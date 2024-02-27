FROM rust:1.76-alpine AS build

# create a new empty shell project
RUN USER=root cargo new --bin efd_to_xlsx
WORKDIR /efd_to_xlsx

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/efd_to_xlsx*
RUN cargo build --release

# our final base
FROM rust:1.76

# copy the build artifact from the build stage
COPY --from=build /efd_to_xlsx/target/release/efd_to_xlsx .

# set the startup command to run your binary
CMD ["./efd_to_xlsx"]