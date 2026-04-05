# 1단계: 빌드 스테이지 (Rust 컴파일러 포함)
FROM rustlang/rust:nightly-slim AS builder

WORKDIR /usr/src/app

# 의존성 캐싱을 위해 설정 파일 먼저 복사함
COPY Cargo.toml Cargo.lock ./
# 빈 소스로 빌드하여 라이브러리만 먼저 다운로드함 (시간 단축용)
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src

# 실제 소스 코드 복사 및 빌드함
COPY . .
RUN cargo build --release

# 2단계: 실행 스테이지 (최소 환경)
FROM debian:bookworm-slim

# 실행에 필요한 라이브러리 및 인증서 설치함
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 빌드 스테이지에서 생성된 바이너리만 가져옴
COPY --from=builder /usr/src/app/target/release/my-docker-term /usr/local/bin/app

# 외부 접속 포트 설정함
EXPOSE 3000

# 서버 실행함
CMD ["app"]

