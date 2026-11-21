# zuwaigani-cli

Great Expectations-style data validation CLI in Rust

## Usage

Commands below run from `crates/cli`. Install the binary with `cargo install --path crates/cli` from the repository root.

```sh
cargo run -- validate --suite samples/passing_suite.yml --output result.json
```

Exit code: `0` all passed, `1` some expectation failed, `2` error.

## Suite

```yaml
name: users_suite
sources:
  polars:
    path: users.csv          # relative to this YAML file
  postgres:
    url_env: DATABASE_URL    # name of the environment variable holding the URL
    table: users             # or schema.table
expectations:
  - type: not_null
    platform: [polars, postgres]   # required; runs once per listed platform
    column: user_id
    mostly: 0.9                    # optional, 0.0 to 1.0
```

Types: `not_null`, `unique`, `between` (`min`, `max`), `in_set` (`values`), `match_regex` (`pattern`).

## Try Postgres locally

Rancher Desktop with the containerd engine uses `nerdctl` instead of `docker`.

```sh
nerdctl run -d --name zuwaigani-pg -e POSTGRES_USER=gx -e POSTGRES_PASSWORD=gx -e POSTGRES_DB=gx \
  -p 127.0.0.1:55432:5432 postgres:16

nerdctl exec -i zuwaigani-pg psql -U gx -d gx -c \
  "CREATE TABLE users (user_id integer, name text, age integer, status text, email text)"
nerdctl exec -i zuwaigani-pg psql -U gx -d gx -c "COPY users FROM STDIN WITH (FORMAT csv, HEADER true)" \
  < samples/users.csv

export DATABASE_URL=postgres://gx:gx@127.0.0.1:55432/gx
cargo run -- validate --suite samples/users_suite.yml
```

## Layout

| Path | Role |
|---|---|
| `src/expectation.rs` | What to check (`Expectation`, `Check`) |
| `src/backend/` | How to check it, one backend per platform; `Backend` trait and `supports()` |
| `src/sources.rs` | Where each platform's data lives |
| `src/suite.rs` | Suite loading, validation and execution |
| `src/result.rs` | The `mostly` pass/fail rule |
| `src/main.rs` | CLI |
| `../core/` | `Platform` and the result types, shared with the frontend |

To add a platform: add a variant to `Platform` in `crates/core`, a source in `sources.rs`, and a module in `src/backend/` with a `supports()` function. The compiler points at every `match` that needs updating.
