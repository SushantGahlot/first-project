FROM rust:1.68 as build

# create a new empty shell project
RUN USER=root cargo new --bin first-project
WORKDIR /first-project

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src
COPY ./migrations ./migrations

# build for release
RUN rm ./target/release/deps/first_project*
RUN cargo build --release

# our final base
FROM debian:buster-slim

RUN apt-get update && apt-get install libpq5 -y
# copy the build artifact from the build stage
COPY --from=build /first-project/target/release/first-project .

ADD https://github.com/ufoscout/docker-compose-wait/releases/download/2.7.3/wait /wait
RUN chmod +x /wait

CMD /wait && ./first-project