# 🚀 Ratel 로컬 개발 환경 설정

## 빠른 시작

### 1. 필수 도구 설치
```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Dioxus CLI
cargo install cargo-binstall && cargo binstall dioxus-cli

# PostgreSQL (macOS)
brew install postgresql@14 && brew services start postgresql@14

# LLVM (WASM 컴파일용)
brew install llvm
```

### 2. 프로젝트 설정
```bash
# 클론
git clone https://github.com/biyard/ratel.git
cd ratel

# 환경 설정
source scripts/setup-local-env.sh

# 데이터베이스 생성
createdb ratel_dev
```

### 3. 서버 실행

**터미널 1 - 백엔드 API 서버:**
```bash
cd ratel
source scripts/setup-local-env.sh
cd packages/main-api && cargo run
```

**터미널 2 - 프론트엔드 서버:**
```bash
cd ratel
source scripts/setup-local-env.sh
cd packages/main-ui && dx serve --fullstack
```

> ⚠️ **중요**: 각 터미널에서 `source scripts/setup-local-env.sh`를 실행해야 합니다!

### 4. 접속 확인
- **Frontend**: http://localhost:8080 (메인 웹 애플리케이션)
- **Backend**: http://localhost:3000 (API 서버)

백엔드가 정상 실행되면 데이터베이스 테이블이 자동으로 생성됩니다.

## 문제 해결

**환경변수 오류 (`You must set ENV`):**
```bash
source scripts/setup-local-env.sh
```

**WASM 컴파일 오류:**
```bash
export CC=/opt/homebrew/opt/llvm/bin/clang
```

**데이터베이스 연결 오류:**
```bash
brew services start postgresql@14
createdb ratel_dev
```

**포트 충돌:**
```bash
lsof -i :3000 :8080
kill -9 <PID>
```

**Firebase 인증 오류 (개발 환경에서는 정상):**
- 로그인 기능은 더미 Firebase 설정으로 인해 오류가 발생하지만 앱 자체는 정상 작동합니다.