FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef

# create a new empty shell project
RUN USER=root cargo new --bin realworld
WORKDIR /realworld

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# copy over your manifests
FROM chef AS builder 
COPY --from=planner /realworld/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo prisma generate
RUN cargo prisma db push
RUN cargo build --release --bin realworld

# our final base
FROM debian:bullseye-slim AS runtime


RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /realworld
COPY --from=builder /realworld/target/release/realworld /usr/local/bin
# set the startup command to run your binary
ENTRYPOINT ["/usr/local/bin/realworld"]