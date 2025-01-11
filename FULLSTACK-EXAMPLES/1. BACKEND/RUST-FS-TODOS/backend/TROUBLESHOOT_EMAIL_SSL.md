

# Inside docker container

```bash
docker logs root-api-1
docker exec -it root-api-1 /bin/bash # Note: Not working.....
apt-get update && apt-get install -y swaks
swaks --to your-email@example.com --from your-smtp-username --server smtp.gmail.com:587 -tls
```
