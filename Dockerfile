####################################################################################################
## Builder
####################################################################################################
FROM rust:alpine3.22 AS builder

RUN apk add --no-cache openssl-dev openssl-libs-static musl-dev

WORKDIR /app

COPY ./ .

RUN cargo build --release

####################################################################################################
## Final image
####################################################################################################
FROM gcr.io/distroless/static-debian12:nonroot

WORKDIR /app

# Copy our build
COPY --from=builder /app/target/release/book_api ./
COPY --from=builder /app/assets ./assets/

ENTRYPOINT ["/app/book_api"]
CMD ["serve"]
