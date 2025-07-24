# Builds a base image that will be used to build the tools.
FROM rust:1.87 AS base
RUN cargo install --locked cargo-chef sccache
ENV RUSTC_WRAPPER=sccache SCCACHE_DIR=/sccache
WORKDIR /app

# Builds a cargo-chef plan
FROM base AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Builds the dependencies using the cargo-chef plan and sccache
FROM base AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json

ARG BUILD_PROFILE=release
ENV BUILD_PROFILE=$BUILD_PROFILE

COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=$SCCACHE_DIR,sharing=locked \
    cargo build --profile $BUILD_PROFILE

# ARG is not resolved in COPY so we have to hack around it by copying the
# binary to a temporary location
RUN cp /app/target/$BUILD_PROFILE/execution-probe /app/execution-probe
RUN cp /app/target/$BUILD_PROFILE/beacon-probe /app/beacon-probe
RUN cp /app/target/$BUILD_PROFILE/blockspeed /app/blockspeed

FROM debian:bookworm-slim AS execution-probe
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /app/execution-probe /usr/local/bin
ENTRYPOINT ["/usr/local/bin/execution-probe"]

FROM debian:bookworm-slim AS beacon-probe
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /app/beacon-probe /usr/local/bin
ENTRYPOINT ["/usr/local/bin/beacon-probe"]

FROM debian:bookworm-slim AS blockspeed
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /app/blockspeed /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/blockspeed"]