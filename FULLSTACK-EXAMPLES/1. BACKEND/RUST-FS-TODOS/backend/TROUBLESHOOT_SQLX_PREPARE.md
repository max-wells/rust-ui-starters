

Note: I've switched to sqlx 0.7.0

```bash
cargo clean
cargo build
cargo sqlx prepare  # Working
```



If error with `docker-compose up`, try this:
```bash
docker-compose down -v
docker-compose up --build
```
