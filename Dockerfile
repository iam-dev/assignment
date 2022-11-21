# stage 1 - generate receipe file for dependencies
FROM rust:1.61.0 as planner

# set the working directory
WORKDIR /app

# install cmake and libpq-dev
RUN USER=root apt-get update && apt-get install -y cmake && apt-get install -y libpq-dev

# install cargo-chef
RUN cargo install cargo-chef

# copy the source code
COPY . .

# generate the recipe file
RUN cargo chef prepare --recipe-path recipe.json

#
# stage 2 - build our dependencies
#
FROM rust:1.61.0 as cacher

# set the working directory
WORKDIR /app

# install cmake and libpq-dev
RUN USER=root apt-get update && apt-get install -y cmake && apt-get install -y libpq-dev

# install cargo-chef
RUN cargo install cargo-chef

# copy from planner
COPY --from=planner /app/recipe.json recipe.json

# run cargo chef cook
RUN cargo chef cook --release --recipe-path recipe.json

#
# stage 3 - build our app
#
FROM rust:1.61.0 as builder

# copy the app into the docker image
COPY . /app

# set the working directory
WORKDIR /app 

# copy from cacher
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

#build the app
RUN cargo build --release

#
# stage 4 - run our app
#
FROM debian:buster-slim

# set the working directory
WORKDIR /app

RUN USER=root apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

# copy from builder
COPY --from=builder /app/target/release/payments-server /app/

# expose the port to the outside world
EXPOSE 50051

# run the app
CMD ["./payments-server"]