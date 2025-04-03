# Imagen oficial de Rust
FROM rust:1.71.0 as builder

# Configurar directorio de trabajo
WORKDIR /usr/src/app

# Copiar archivos del proyecto
COPY . .

# Compilar el proyecto en modo release
RUN cargo build --release

# Imagen final
FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/rust_postgres_docker .

# Variables de entorno
ENV DATABASE_URL postgres://postgres:password@localhost:5432/basesita

CMD ["./rust_postgres_docker"]
