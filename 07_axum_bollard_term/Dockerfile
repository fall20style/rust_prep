# 1단계: 빌드 스테이지 (최신 Cargo.lock 및 Edition 2024 대응을 위해 nightly 사용함)
FROM rustlang/rust:nightly-bookworm AS builder

WORKDIR /usr/src/app

# 빌드에 필요한 시스템 라이브러리 설치함
RUN apt-get update && apt-get install -y libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*

# 프로젝트 파일 전체 복사함 (index.html, xterm.js 등 리소스 포함 필수임)
COPY . .

# Release 모드로 빌드함 (바이너리 이름: my-docker-term)
RUN cargo build --release


# 2단계: 실행 스테이지 (경량화된 데비안 이미지 사용함)
FROM debian:bookworm-slim

# 실행 시 필요한 최소 라이브러리 및 인증서 설치함
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 빌드 스테이지에서 생성된 바이너리를 실행 경로로 복사함
COPY --from=builder /usr/src/app/target/release/my-docker-term /usr/local/bin/my-docker-term

# 실행 권한 부여함
RUN chmod +x /usr/local/bin/my-docker-term

# 컨테이너 외부 접속 포트 설정함 (Axum 기본 포트임)
EXPOSE 3000

# 환경 변수 설정 (로그 확인 및 주소 바인딩 도움용임)
ENV RUST_LOG=debug
ENV RUST_BACKTRACE=1

# 실행 명령어 (절대 경로 사용함)
ENTRYPOINT ["/usr/local/bin/my-docker-term"]

