
# gRPC ZKP

## Run gRPC server
```
cargo run --bin auth-server
```

## Run gRPC client
```
cargo run --bin auth-client
```

# Functional test

Functional test with prime number 10009
p = 31
b = 2
g = 2
c = 17
x = 624
k = 492

## Run functional test server
```
cargo run --bin server-test
```


## Run functional test client
```
cargo run --bin client-test
```


# Run with docker-compose
```
docker-compose up -d
```

# Run with docker update image
```
docker-compose up --force-recreate --build -d
docker image prune -f
```