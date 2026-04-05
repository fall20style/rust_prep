# 1. 빌드 스테이지
FROM rust:1.80-slim AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# 2. 실행 스테이지 (Debian으로 원복)
FROM debian:bookworm-slim

# 필요한 라이브러리와 도커 CLI 설치
RUN apt-get update && apt-get install -y \
    docker.io \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin
# 파일명 axum_bash_cli 인지 다시 확인!
COPY --from=builder /usr/src/app/target/release/axum-bash-cli ./app
RUN chmod +x ./app

EXPOSE 8080
CMD ["./app"]

