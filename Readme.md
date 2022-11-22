
# gRPC ZKP

Run gRPC server
```
cargo run --bin server
```

Run gRPC client
```
cargo run --bin client
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