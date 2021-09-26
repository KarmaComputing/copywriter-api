# Dockerfile.distroless
  
ARG BASE_IMAGE=rust:1.54.0-slim-buster


FROM $BASE_IMAGE as builder
WORKDIR app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian10
COPY --from=builder /app/target/release/copywrighter /
CMD ["./copywrighter"]
EXPOSE 8080
