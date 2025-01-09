FROM rust:1.72

WORKDIR /app

COPY . .

CMD ["sh", "-c", "echo 'El contenedor está listo para compilar archivos Rust'"]