
FROM rust:1.85 as builder

# 
RUN apt-get update && apt-get install -y tzdata \
    && ln -fs /usr/share/zoneinfo/Europe/Moscow /etc/localtime \
    && dpkg-reconfigure -f noninteractive tzdata

WORKDIR /app

# Копируем все исходники проекта в контейнер
COPY . .

# Собираем релиз (укажи путь до манифеста server/Cargo.toml, если сервер в поддиректории)
RUN cargo build --release --manifest-path server/Cargo.toml

# Минимальный runtime-образ на debian slim
FROM debian:buster-slim

RUN apt-get update && apt-get install -y ca-certificates tzdata \
    && ln -fs /usr/share/zoneinfo/Europe/Moscow /etc/localtime \
    && dpkg-reconfigure -f noninteractive tzdata \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Копируем собранный бинарник из builder
COPY --from=builder /app/target/release/server /usr/local/bin/server

EXPOSE 3000

CMD ["server"]

