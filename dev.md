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

generate jwt

``` nex
# zh 生成用户 id=1、过期时间 100 年的 JWT token
# en generate a JWT token for user_id=1 with 100-year expiration
#
# zh 使用 etc/config.toml 中 [jwt] 配置：
# en using the [jwt] config from etc/config.toml:
#   secret = "replace-this-secret"
#   expiration_secs = 3600   # zh 当前配置覆盖 100 年, 仅本次手动生成   en current value is overridden to 100 years for this manual token only
#
# zh 算法: HS256，签名密钥: replace-this-secret
# en algorithm: HS256, signing secret: replace-this-secret
# zh Claims: {"user_id":1,"exp":4945858056}  (exp ≈ 2126-09-23)
# en Claims: {"user_id":1,"exp":4945858056}   (exp ≈ 2126-09-23)
eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyX2lkIjoxLCJleHAiOjQ5NDU4NTgwNTZ9.wxBJ84yhnXIAv1ySdERX3TE3vp-ZfGiJJnj1l5n3OfA
# zh 用法: Authorization: Bearer <token>
# en usage: Authorization: Bearer <token>
```
