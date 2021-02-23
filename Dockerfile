FROM rust:1.50

WORKDIR app
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release
ENTRYPOINT [ "./target/release/newsletter" ]
