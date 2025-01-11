YT: https://www.youtube.com/watch?v=X74uyTr5kk8





```bash
# .env
DATABASE_URL=postgresql://postgres:password@localhost:5432/rust_deploy
JWT_SECRET=my_ultra_secure_jwt_secret_key
JWT_MAXAGE=60
```



```bash
# sqlx migrate add -r user_table   (already done)
sqlx migrate run
```
