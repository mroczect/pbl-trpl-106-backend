# pbl-trpl-106-backend

Full backend implementation in Rust for the Galonku application (Water Depot Management), built for Project-Based Learning (TRPL 106).

---


## Prerequisites

- [Rust](https://rustup.rs/)
- [Podman](https://podman.io/) atau Docker

--

## Runing

1. buat `.env`

```sh
$ cp .env.example .env
```

```env
# ini setrah klen aja atur atur aja sukak klen
# 
PORT=3000
APP_MODE=dev
# APP_MODE=prod
# APP_VERSION=1.0.0
DATABASE_URL=mysql://root:root@127.0.0.1:3307/pbl_trpl_106
```
nah di `APP_MODE` ini harus di definiskan ya klen bisa tengok aja di `src/app/config.rs`

nnti isinya ad akayak gini

```rs
impl Mode {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Dev => "dev",
            Self::Prod => "prod",
        }
    }
}
```

nah bednaya giniaja


kalau dev mode url nya pakek git hash teakhir contoh : `/api/vbd20468-dev/hello`
kalau prod mode, dia pakek yang dari `.env` contoh : `/api/v1.0.0/hello`

---

## End to end tests

nah kalau udah smeuana cara run full nya itu simpenya gini ja

```sh
$ cargo fmt --all && cargo check

# pastiin ga ada erorr atau apa

# siapin container
$ podman start pbl-mysql

# Testing

podman exec pbl-mysql mysql -uroot -proot -e "SHOW TABLES" pbl_trpl_106

# kalau udah

$ cargo run

# kalau smeuanay udah lancar run smoke sh

# $ chmod +x scripts/smoke.sh (opsional aja kalau error ngak bsia di run)
# $ bash scripts/smoke.sh
# 
./scripts/smoke.sh
```

## SELESAI