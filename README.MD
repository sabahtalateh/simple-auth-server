## Run container for database 
```docker
docker run --name rust_auth_postgres -p 54333:5432 -e POSTGRES_PASSWORD=root -d postgres
```

## Create Psql DB
```docker
docker exec -it rust_auth_postgres psql -U postgres -c "CREATE DATABASE auth WITH ENCODING 'UTF8'"
```

## Create Psql user
```docker
docker exec -it rust_auth_postgres psql -U postgres -c "CREATE USER auth WITH ENCRYPTED PASSWORD 'auth'"
```

## Grant access to db to user
```docker
docker exec -it rust_auth_postgres psql -U postgres -c "GRANT ALL PRIVILEGES ON DATABASE auth TO auth"
```
```docker
docker exec -it rust_auth_postgres psql -U postgres -c "ALTER DATABASE auth OWNER TO auth"
```
