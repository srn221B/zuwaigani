# zuwaigani

Great Expectations-style data validation in Rust.

| Path | Role |
|---|---|
| [`crates/cli`](crates/cli) | The `zuwaigani` command. Runs on its own; see its README |
| [`crates/core`](crates/core) | Types shared by the CLI and the frontend. Depends only on serde and builds for wasm |
| [`frontend`](frontend) | Leptos web UI (work in progress) |

The CLI and the frontend never depend on each other, only on `crates/core`.

## CLI Usecase

```sh
$ zuwaigani validate --suite crates/cli/samples/users_suite.yml 
suite: users_suite
  **NG** polars    expect_column_values_to_be_unique        user_id      unexpected 2/7 (28.57%)
      example: 5, 5
  **NG** postgres  expect_column_values_to_be_unique        user_id      unexpected 2/7 (28.57%)
      example: 5, 5
  **OK** polars    expect_column_values_to_not_be_null      age          unexpected 0/8 (0.00%)
FAILED  (1/3 passed)
```

## Architecture

TODO(mermaid)

## Develop

```sh
make ci            # CLI and core: fmt, clippy, tests
make frontend-ci   # frontend: ssr and wasm builds
make frontend-dev  # frontend: dev server with hot reload (http://127.0.0.1:3000)
make capabilities  # regenerate frontend/data/capabilities.json after changing what a platform supports
make daisyui       # fetch the daisyUI plugin pinned in the Makefile (frontend/style/daisyui.mjs)
make deploy        # install the zuwaigani command
```
