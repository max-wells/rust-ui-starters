TUTORIEL (Lets Get Rusty): https://www.youtube.com/watch?v=_gMzg77Qjm0 (20min)
REPOSITORY: https://github.com/letsgetrusty/api-deployment-example



Make sure to have a `.env` file:
```bash
DATABASE_URL=postgresql://postgres:password@localhost:5432/rust_deploy
POSTGRES_PASSWORD=password
```

```bash
docker-compose up
```




# 2. DEPLOY

Go to Digital Ocean > Dashboard > Create Droplet


Don't forget to open the console of your droplet and enter these commands:
- sudo apt-get update
- sudo apt-get install docker.io
- sudo apt-get install docker-compose
====> docker --version
====> docker-compose --version



If you have problem with docker-compose, try this:
```bash
sudo curl -L "https://github.com/docker/compose/releases/latest/download/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
# 1. First, make the downloaded Docker Compose executable:
sudo chmod +x /usr/local/bin/docker-compose
# 2. Now, try running Docker Compose with the full path:
/usr/local/bin/docker-compose --version
# 3. If that works, we need to make sure the system can find Docker Compose. You can do this by creating a symbolic link
sudo ln -s /usr/local/bin/docker-compose /usr/bin/docker-compose
docker-compose --version
```

=====> docker-compose --version ✅





# 3. GITHUB ACTIONS

Go to your repo > Settings > Secrets and variables > Actions


## Secrets
DOCKER_USERNAME --> everlabs
DOCKER_PASSWORD --> XXXXXX
DROPLET_PASSWORD ---> cf. Root password (BitWarden)
POSTGRES_PASSWORD --> password
DO_TOKEN ---> Go to Digital Ocean > API > Generate a Personal Access Token

## Variables
DROPLET_IP --> XXXX



---> Get your IP address on Digital Ocean
(GET) http://209.38.144.246:8000/
(GET) http://209.38.144.246:8000/users


Then, just verifiy in the DRoplet console:
```bash
docker ps
```



