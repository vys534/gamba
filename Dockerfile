FROM lukemathwalker/cargo-chef:latest-rust-1.70.0 AS chef
WORKDIR /usr/src/gamba

FROM chef AS prepare
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS build
COPY --from=prepare /usr/src/gamba/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM rust AS runtime
COPY --from=build /usr/src/gamba/target/release/gamba .
CMD ["./gamba"]