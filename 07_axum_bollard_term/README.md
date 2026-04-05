
## Axum-xterm.js 기반 Docker Web Terminal

- 비동기 이벤트 기반의 스트리밍 프록시 구조

### 시스템 배치도 (Deployment Diagram)

```mermaid
graph TD
    subgraph UserPC [User Device]
        Browser[Web Browser <br/> xterm.js]
    }

    subgraph Host [Host Server / Docker Host]
        subgraph RustApp [Axum Server Process]
            Axum[Axum Framework]
            Bollard[Bollard Docker API]
        end

        subgraph DockerEngine [Docker Daemon]
            UnixSock((/var/run/docker.sock))
        end

        subgraph Container [Target Container]
            Bash[bash shell]
        end
    end

    Browser <-->|WebSocket / Port 3000| Axum
    Axum <--> Bollard
    Bollard <-->|Unix Socket| UnixSock
    UnixSock <-->|Exec / Stream| Bash

    style Browser fill:#f9f,stroke:#333,stroke-width:2px
    style Axum fill:#f96,stroke:#333,stroke-width:2px
    style Bash fill:#9f9,stroke:#333,stroke-width:2px
    style UnixSock fill:#ddd,stroke:#333
```

### 1. 시스템 개요 (System Overview)
웹 브라우저에서 실행 중인 Docker 컨테이너의 셸(bash)에 직접 접속하여 명령어를 실행하고 결과를 실시간으로 확인할 수 있는 Web-based 원격 터미널.

### 2. 기술 스택 (Tech Stack)

* Frontend: xterm.js (터미널 렌더링), 브라우저 표준 WebSocket API
* Backend: Rust (Axum Framework), Tokio (비동기 런타임)
* Infrastructure: Docker (Target Container), Bollard (Docker API Client)
* Communication: WebSocket (Binary/Text Streaming)

### 3. 시스템 아키텍처 (Architecture)## 3.1. 논리적 계층 구조 (Logical Layers)

1. UI Layer (Browser): 사용자의 키 입력을 가로채 서버로 보내고, 서버에서 온 바이트 데이터를 ANSI 시퀀스로 해석하여 화면에 그림.
2. Streaming Layer (Axum WS): WebSocket 세션을 유지하며 브라우저와 서버 간의 전송 채널 제공.
3. Bridge Layer (Rust/Bollard): WebSocket의 입출력 스트림과 Docker Exec의 stdin/stdout 스트림을 상호 매핑(Forwarding).
4. Runtime Layer (Docker Engine): 실제 격리된 환경에서 bash 프로세스를 구동하고 TTY를 할당.

### 3.2. 데이터 흐름도 (Data Flow)

* Input: User Key → xterm.js → WebSocket (Binary) → Axum → Docker Stdin
* Output: Docker Stdout/err → Axum → WebSocket (Binary) → xterm.js → Screen Update

### 4. 상세 설계 (Detailed Design)## 4.1. WebSocket 핸들링 및 비동기 처리

* Task 분리: 하나의 WebSocket 연결에 대해 두 개의 비동기 Task(tokio::spawn)를 생성하여 전이중(Full-duplex) 통신을 구현함.
* docker_to_ws: 컨테이너의 출력을 계속 감시하며 브라우저로 쏴줌.
   * ws_to_docker: 브라우저의 입력을 컨테이너의 입력 스트림에 씀.
* 종료 처리: tokio::select! 매크로를 사용하여 한쪽 세션이 끊기면(브라우저를 닫거나 컨테이너가 죽으면) 즉시 리소스를 해제함.

### 4.2. TTY 및 환경 설정

* Interactive Mode: CreateExecOptions에서 tty: true, stdin: true를 설정하여 대화형 프롬프트 활성화.
* 컬러 및 인코딩: TERM=xterm-256color 환경 변수를 주입하여 리눅스 표준 컬러 출력을 지원함.

### 5. 주요 인터페이스 (API Interface)

| 구분 | 프로토콜 | 엔드포인트 | 역할 |
|---|---|---|---|
| Static | HTTP/GET | / | 터미널 클라이언트(index.html) 제공 |
| Assets | HTTP/GET | /xterm.js | xterm.js 라이브러리 서빙 |
| Stream | WebSocket | /ws | 실제 셸 데이터 스트리밍 |

### 6. 보안 및 고려사항 (Security & Considerations)

* 권한 관리: 현재 시스템은 /var/run/docker.sock에 직접 접근하므로, 서버 프로세스의 보안 권한 관리가 핵심임.
* 리소스 관리: 사용자가 접속할 때마다 docker exec 프로세스가 생성되므로, 비정상 종료 시에도 컨테이너 내 프로세스가 좀비가 되지 않도록 abort() 처리를 강화함.

### 디버깅
- 검은 화면에서 멈춤
  - xterm.js, xterm.css를 받아서 local에 저장함

