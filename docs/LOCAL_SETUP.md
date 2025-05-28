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
```bash
# 터미널 1: 백엔드
cd packages/main-api && cargo run

# 터미널 2: 프론트엔드  
source scripts/setup-local-env.sh
cd packages/main-ui && dx serve --fullstack
```

### 4. 접속
- Frontend: http://localhost:8080
- Backend: http://localhost:3000

## 문제 해결

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