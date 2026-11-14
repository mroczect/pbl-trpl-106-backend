#!/usr/bin/env bash

set -uo pipefail

if [ "${1:-}" = -h ] || [ "${1:-}" = --help ]; then
    cat <<'EOF'
env:
  PORT=3002             listen port (default 3002, matches src/app/config.rs)
  BASE=URL              override full base URL
  APP_MODE=dev|prod     from .env if present
  APP_VERSION=x.y.z     required when APP_MODE=prod
  SPAWN=1               spawn `cargo run` if server not up
  RUST_LOG=...          passed to spawned server (default info,tower_http=debug)
  SERVER_LOG=path       file to tail on failure (auto-set by SPAWN)
  TAIL_N=40             lines of SERVER_LOG to tail
  V=1                   verbose: echo every request/response
  LOG=path              write client-side log

note: backend request logs need RUST_LOG=info,tower_http=debug
exit: 0 all pass, 1 any fail
EOF
    exit 0
fi

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]:-$0}")" && pwd)
cd "$SCRIPT_DIR/.." || { echo "cannot cd to repo root"; exit 1; }

if [ -f .env ]; then
    set -a
    . ./.env
    set +a
fi

PORT=${PORT:-3002}
BASE=${BASE:-http://localhost:$PORT}
MODE=${APP_MODE:-dev}
V=${V:-0}
TAIL_N=${TAIL_N:-40}
LOG=${LOG:-}

case "$MODE" in
    dev|development) MODE=dev ;;
    prod|production) MODE=prod ;;
    *) echo "ERROR: APP_MODE must be dev|prod, got '$MODE'"; exit 1 ;;
esac

if [ "$MODE" = prod ]; then
    if [ -z "${APP_VERSION:-}" ]; then
        echo "ERROR: APP_MODE=prod requires APP_VERSION"
        exit 1
    fi
    VERSION=$APP_VERSION
else
    VERSION="$(git rev-parse --short HEAD 2>/dev/null || echo unknown)-dev"
fi
PREFIX="/api/v$VERSION"

PASS=0
FAIL=0
FAIL_LIST=""
SPAWNED_PID=""

log() { [ -n "$LOG" ] && printf '%s\n' "$*" >> "$LOG"; return 0; }
pass() { echo "PASS $1"; PASS=$((PASS+1)); log "PASS $1"; }

fail() {
    echo "FAIL $1"
    FAIL=$((FAIL+1))
    FAIL_LIST="${FAIL_LIST}  - $1"$'\n'
    log "FAIL $1"
}

# check <desc> <want> <path> [method] [body] [substr]
check() {
    _desc=$1; _want=$2; _path=$3
    _method=${4:-GET}; _body=${5:-}; _sub=${6:-}

    _url="$BASE$_path"
    _args=(-sS --max-time 10 -w $'\n---HTTP:%{http_code}---TIME:%{time_total}---')
    [ "$_method" != GET ] && _args+=(-X "$_method")
    [ -n "$_body" ] && _args+=(-H 'content-type: application/json' --data-binary "$_body")

    log "REQ $_method $_url body=${_body:-<none>}"

    if ! _out=$(curl "${_args[@]}" "$_url" 2>&1); then
        fail "$_desc (curl error: $_out)"
        echo "     cmd: curl -sS '$_url'"
        return
    fi

    _code=${_out##*---HTTP:}
    _code=${_code%%---TIME:*}
    _time=${_out##*---TIME:}
    _time=${_time%%---*}
    _resp=${_out%$'\n'---HTTP:*}

    log "RES $_code ${_time}s body=$_resp"
    [ "$V" = 1 ] && printf '  -> %s %s => %s (%ss)\n' "$_method" "$_url" "$_code" "$_time"

    if [ "$_code" != "$_want" ]; then
        fail "$_desc (want=$_want got=$_code)"
        echo "     cmd: curl -sS '$_url'${_body:+ -H 'content-type: application/json' -d '$_body'}"
        echo "     body: $_resp"
        return
    fi

    if [ -n "$_sub" ] && [ "${_resp#*"$_sub"}" = "$_resp" ]; then
        fail "$_desc (body missing '$_sub')"
        echo "     body: $_resp"
        return
    fi

    pass "$_desc ($_code)"
}

wait_ready() {
    _i=0
    while [ "$_i" -lt 20 ]; do
        curl -sf -o /dev/null --max-time 1 "$BASE/health" && return 0
        sleep 0.5
        _i=$((_i+1))
    done
    return 1
}

spawn_server() {
    [ "${SPAWN:-0}" = 1 ] || return 0
    if curl -sf -o /dev/null --max-time 1 "$BASE/health" 2>/dev/null; then
        echo "note: server already up on $BASE (SPAWN ignored)"
        return 0
    fi
    export SERVER_LOG=${SERVER_LOG:-/tmp/smoke-server.log}
    export RUST_LOG=${RUST_LOG:-info,tower_http=debug}
    : > "$SERVER_LOG"
    echo "spawning: RUST_LOG=$RUST_LOG cargo run > $SERVER_LOG"
    cargo run > "$SERVER_LOG" 2>&1 &
    SPAWNED_PID=$!
    echo "  pid=$SPAWNED_PID"
}

cleanup() {
    if [ -n "$SPAWNED_PID" ]; then
        kill "$SPAWNED_PID" 2>/dev/null
        wait "$SPAWNED_PID" 2>/dev/null
        SPAWNED_PID=""
    fi
}
trap cleanup EXIT INT TERM

port_hint() {
    echo
    echo "  tried : $BASE/health"
    for _p in 3000 3001 3002 8080; do
        if curl -sf -o /dev/null --max-time 1 "http://localhost:$_p/health" 2>/dev/null; then
            echo "  live  : port $_p responds — try PORT=$_p $0"
        fi
    done
    echo "  hint  : backend default port is 3002 (src/app/config.rs)"
    echo "  hint  : request logs need RUST_LOG=info,tower_http=debug"
}

check_binary_version() {
    _out=$(curl -sS --max-time 5 "$BASE$PREFIX/info" 2>/dev/null) || return 0
    _srv=""
    case "$_out" in
        *'"version":"'*'"'*)
            _srv=${_out#*\"version\":\"}
            _srv=${_srv%%\"*}
            ;;
    esac
    if [ -n "$_srv" ] && [ "$_srv" != "$VERSION" ]; then
        echo "WARN binary reports version=$_srv but script expects $VERSION"
        echo "     rebuild? cargo build  (GIT_HASH baked at compile time by build.rs)"
    fi
}

dump_server_log() {
    [ -n "${SERVER_LOG:-}" ] && [ -f "$SERVER_LOG" ] || return 0
    echo
    echo "== server log tail (last $TAIL_N lines of $SERVER_LOG)"
    tail -n "$TAIL_N" "$SERVER_LOG"
}

echo "base=$BASE mode=$MODE version=$VERSION prefix=$PREFIX"
echo

spawn_server

echo "== [0] server ready"
if ! wait_ready; then
    fail "server not listening on $BASE"
    port_hint
    dump_server_log
    exit 1
fi
pass "server listening"
check_binary_version
echo

echo "== [1] health"
check "GET /health -> 200"          200 /health
check "GET /live -> 200"            200 /live
check "GET /ready -> 200"           200 /ready
check "GET /health status:ok"       200 /health GET '' '"status":"ok"'
check "GET /live body ok"           200 /live   GET '' 'ok'
echo

echo "== [2] business api"
check "GET hello"                   200 "$PREFIX/hello" GET '' '"message":"hello from rust backend"'
check "GET info mode"               200 "$PREFIX/info"  GET '' "\"mode\":\"$MODE\""
check "GET info version"            200 "$PREFIX/info"  GET '' "\"version\":\"$VERSION\""
check "GET info api_prefix"         200 "$PREFIX/info"  GET '' "\"api_prefix\":\"$PREFIX\""
check "POST echo ok"                200 "$PREFIX/echo"  POST '{"message":"halo"}' '"message":"halo"'
check "POST echo blank -> 400"      400 "$PREFIX/echo"  POST '{"message":"  "}'
echo

echo "== [3] error demo"
check "GET /error/validation -> 400"   400 "$PREFIX/error/validation"   GET '' '"code":"VALIDATION"'
check "GET /error/unauthorized -> 401" 401 "$PREFIX/error/unauthorized" GET '' '"code":"UNAUTHORIZED"'
check "GET /error/conflict -> 409"     409 "$PREFIX/error/conflict"     GET '' '"code":"CONFLICT"'
check "GET /error/not-found -> 404"    404 "$PREFIX/error/not-found"    GET '' '"code":"NOT_FOUND"'
check "GET /error/internal -> 500"     500 "$PREFIX/error/internal"     GET '' '"code":"INTERNAL"'
echo

echo "== [4] negative prefix"
check "GET /v1/api/hello -> 404"      404 /v1/api/hello
check "GET wrong hash -> 404"         404 /api/vwronghash-dev/hello
if [ "$MODE" = dev ]; then
    check "APP_VERSION ignored in dev -> 404" 404 /api/v1.0.0/info
fi
echo

echo "total=$((PASS+FAIL)) pass=$PASS fail=$FAIL"
if [ "$FAIL" -gt 0 ]; then
    echo
    echo "failures:"
    printf '%s' "$FAIL_LIST"
    dump_server_log
    exit 1
fi
