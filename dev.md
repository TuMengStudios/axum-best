# dev tools

``` nex
# commit message linter (Go based, requires Go toolchain; invoked by pre-commit)
go install github.com/conventionalcommit/commitlint@v0.10.1
# hot reload subcommand
cargo install cargo-watch
# test subcommand
cargo install cargo-nextest --locked
```

develop

``` nex
# zh 热重载开发测试
# en hot reload development test
cargo watch -x
```

test

``` nex
# zh 运行测试
# en run test case
cargo nextest run
```
