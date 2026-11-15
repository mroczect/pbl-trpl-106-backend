#!/usr/bin/env bash

set -uo pipefail

if [ "${1:-}" = -h ] || [ "${1:-}" = --help ]; then
    cat <<'EOF'
env:
  SERVE_CONTAINER=pbl-mysql     container name
  SERVE_DB=pbl_trpl_106         database name
  SERVE_DB_USER=root            db user for healthcheck and reset
  SERVE_DB_PASS=root            db password (defaults to MYSQL_ROOT_PASSWORD)
  HEALTH_TIMEOUT=60             seconds to wait for db healthy

options:
  --reset-db          drop and recreate database (data lost, container kept)
  --fresh-container   remove container and volume, recreate via compose
  --release           cargo run --release
  --skip-check        skip cargo check
  -h, --help          show this help

exit: 0 ok, 1 error, 130 canceled
EOF
    exit 0
fi

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]:-$0}")" && pwd)
cd "$SCRIPT_DIR/.." || { echo "cannot cd to repo root"; exit 1; }

DO_RESET=0
DO_FRESH=0
DO_RELEASE=0
DO_SKIP_CHECK=0

for arg in "$@"; do
    case "$arg" in
        --reset-db)        DO_RESET=1 ;;
        --fresh-container) DO_FRESH=1 ;;
        --release)         DO_RELEASE=1 ;;
        --skip-check)      DO_SKIP_CHECK=1 ;;
        *) echo "ERROR: unknown option '$arg' (use --help)"; exit 1 ;;
    esac
done

log()  { printf '== %s\n' "$*"; }
warn() { printf 'WARN: %s\n' "$*" >&2; }
die()  { printf 'ERROR: %s\n' "$*" >&2; exit 1; }

confirm() {
    printf '%s [y/N] ' "$1"
    read -r _ans || _ans=""
    case "$_ans" in
        y|Y|yes|YES) return 0 ;;
        *) return 1 ;;
    esac
}

command -v cargo  >/dev/null || die "cargo not found"
command -v podman >/dev/null || die "podman not found"

[ -f .env ] || die ".env not found (copy from .env.example)"

set -a
. ./.env
set +a

[ -n "${JWT_SECRET:-}" ] || die "JWT_SECRET empty in .env"
[ "${#JWT_SECRET}" -ge 32 ] || die "JWT_SECRET must be >= 32 chars (got ${#JWT_SECRET})"
[ -n "${DATABASE_URL:-}" ] || die "DATABASE_URL empty in .env"

CONTAINER=${SERVE_CONTAINER:-pbl-mysql}
DB=${SERVE_DB:-pbl_trpl_106}
DB_USER=${SERVE_DB_USER:-root}
DB_PASS=${SERVE_DB_PASS:-${MYSQL_ROOT_PASSWORD:-root}}
HEALTH_TIMEOUT=${HEALTH_TIMEOUT:-60}

echo "container=$CONTAINER db=$DB timeout=${HEALTH_TIMEOUT}s"

if [ "$DO_FRESH" = 1 ]; then
    warn "fresh container will delete all database data"
    confirm "continue?" || { echo "canceled"; exit 130; }
    if command -v podman-compose >/dev/null && [ -f compose.yaml ]; then
        podman-compose down -v || true
        podman-compose up -d || die "podman-compose up failed"
    else
        podman stop "$CONTAINER" 2>/dev/null
        podman rm -f "$CONTAINER" 2>/dev/null
        podman volume ls --format '{{.Name}}' | while read -r _v; do
            case "$_v" in
                *mysql*) podman volume rm "$_v" 2>/dev/null || true ;;
            esac
        done
        die "container removed; recreate manually or run with compose.yaml"
    fi
    log "container recreated"
fi

container_running() {
    podman ps --format '{{.Names}}' 2>/dev/null | grep -qx "$CONTAINER"
}

container_healthy() {
    _status=$(podman inspect --format '{{if .State.Health}}{{.State.Health.Status}}{{else}}none{{end}}' "$CONTAINER" 2>/dev/null || echo none)
    case "$_status" in
        healthy) return 0 ;;
        none)
            podman exec "$CONTAINER" mysqladmin ping -h 127.0.0.1 -u"$DB_USER" -p"$DB_PASS" --silent >/dev/null 2>&1
            return $?
            ;;
        *) return 1 ;;
    esac
}

if ! container_running; then
    log "starting container $CONTAINER"
    if command -v podman-compose >/dev/null && [ -f compose.yaml ]; then
        podman-compose up -d || die "podman-compose up failed"
    else
        podman start "$CONTAINER" >/dev/null || die "podman start failed"
    fi
fi

log "waiting for db healthy"
_elapsed=0
while ! container_healthy; do
    if [ "$_elapsed" -ge "$HEALTH_TIMEOUT" ]; then
        printf '\n'
        die "db not healthy after ${HEALTH_TIMEOUT}s (check: podman logs $CONTAINER)"
    fi
    printf '.'
    sleep 1
    _elapsed=$((_elapsed+1))
done
printf '\n'
log "db healthy (${_elapsed}s)"

if [ "$DO_RESET" = 1 ]; then
    warn "reset will drop and recreate database $DB"
    confirm "continue?" || { echo "canceled"; exit 130; }
    podman exec "$CONTAINER" mysql -u"$DB_USER" -p"$DB_PASS" -e \
        "DROP DATABASE IF EXISTS \`$DB\`; CREATE DATABASE \`$DB\` CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;" \
        || die "reset failed"
    log "db reset"
fi

if [ "$DO_SKIP_CHECK" = 0 ]; then
    log "cargo check"
    cargo check --quiet || die "cargo check failed"
fi

log "starting server"
if [ "$DO_RELEASE" = 1 ]; then
    exec cargo run --release
else
    exec cargo run
fi
