FROM rust:alpine as build
RUN apk add musl-dev
WORKDIR /workdir
COPY . .
RUN cargo build --release

FROM alpine
COPY --from=build /workdir/target/release/axum-htmx-maud-tailwind .
CMD ["./axum-htmx-maud-tailwind"]