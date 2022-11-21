

https://medium.com/asecuritysite-when-bob-met-alice/rust-and-bulletproofs-towards-a-more-trusted-and-private-digital-world-6120dd721390


# gRPC example in Rust

Run gRPC server
```
cargo run --bin payments-server
```

Run gRPC client
```
cargo run --bin payments-client
```

# Run with docker-compose
```
docker-compose up -d -f
```

## Run Postgres db
```
docker-compose up -d -f docker-compose-db.yml
```

# Run with docker update image
```
docker-compose up --force-recreate --build -d
docker image prune -f
```