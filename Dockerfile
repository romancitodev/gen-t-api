# Establecer la imagen base de Rust
FROM rust:latest as builder

# Establecer el directorio de trabajo
WORKDIR /app

# Copiar el archivo Cargo.toml y el archivo Cargo.lock
COPY Cargo.toml Cargo.lock ./

# Descargar las dependencias sin compilar
RUN mkdir src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release

# Copiar el resto de los archivos del proyecto
COPY . .

# Compilar la aplicación
RUN cargo build --release

# Establecer la imagen base de producción
FROM debian:buster-slim

# Instalar dependencias de tiempo de ejecución para Redis
RUN apt-get update \
    && apt-get install -y redis

# Copiar la aplicación compilada desde el contenedor de compilación
COPY --from=builder /app/target/release/gen-t-api /home/gen-t-api

# Exponer el puerto de Redis
EXPOSE 6379

# Exponer el puerto de Rocket
EXPOSE 8000

# Ejecutar la aplicación
CMD ["/home/gen-t-api"]