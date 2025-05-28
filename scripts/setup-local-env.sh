#!/bin/bash

# Ratel 로컬 개발 환경 설정 스크립트
# Local Development Environment Setup Script for Ratel

set -e  # 에러 발생 시 스크립트 중단

echo "🚀 Ratel 로컬 개발 환경 설정 중..."
echo "🚀 Setting up Ratel local development environment..."

# 색상 정의
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 필수 도구 확인 함수
check_prerequisites() {
    echo "🔍 필수 도구 확인 중..."
    echo "🔍 Checking prerequisites..."
    
    # Rust 확인
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}❌ Rust가 설치되지 않았습니다. https://rustup.rs/ 에서 설치하세요.${NC}"
        echo -e "${RED}❌ Rust is not installed. Please install from https://rustup.rs/${NC}"
        exit 1
    fi
    
    # Dioxus CLI 확인
    if ! command -v dx &> /dev/null; then
        echo -e "${RED}❌ Dioxus CLI가 설치되지 않았습니다.${NC}"
        echo -e "${RED}❌ Dioxus CLI is not installed.${NC}"
        echo "설치 명령어 / Install command:"
        echo "cargo install cargo-binstall && cargo binstall dioxus-cli"
        exit 1
    fi
    
    # PostgreSQL 확인
    if ! command -v psql &> /dev/null; then
        echo -e "${YELLOW}⚠️  PostgreSQL이 설치되지 않았습니다. 설치를 권장합니다.${NC}"
        echo -e "${YELLOW}⚠️  PostgreSQL is not installed. Installation is recommended.${NC}"
        echo "macOS: brew install postgresql@14"
        echo "Ubuntu: sudo apt-get install postgresql-14"
    fi
    
    # LLVM 확인 (WASM 컴파일용)
    if ! command -v /opt/homebrew/opt/llvm/bin/clang &> /dev/null; then
        echo -e "${YELLOW}⚠️  LLVM이 설치되지 않았습니다. WASM 컴파일에 필요합니다.${NC}"
        echo -e "${YELLOW}⚠️  LLVM is not installed. Required for WASM compilation.${NC}"
        echo "macOS: brew install llvm"
    fi
    
    echo -e "${GREEN}✅ 필수 도구 확인 완료${NC}"
    echo -e "${GREEN}✅ Prerequisites check completed${NC}"
}

# 데이터베이스 설정 함수
setup_database() {
    echo "🗄️  데이터베이스 설정 중..."
    echo "🗄️  Setting up database..."
    
    # PostgreSQL 서비스 시작 (macOS)
    if [[ "$OSTYPE" == "darwin"* ]]; then
        if command -v brew &> /dev/null; then
            brew services start postgresql@14 2>/dev/null || true
        fi
    fi
    
    # 데이터베이스 생성
    if command -v psql &> /dev/null; then
        createdb ratel_dev 2>/dev/null || echo "데이터베이스가 이미 존재하거나 생성할 수 없습니다."
        echo "Database already exists or cannot be created."
    fi
}

# 환경변수 설정
setup_environment() {
    echo "⚙️  환경변수 설정 중..."
    echo "⚙️  Setting up environment variables..."
    
    # 기본 환경 설정
    export ENV=dev
    export DOMAIN=dev.ratel.foundation
    export AUTH_DOMAIN=dev.ratel.foundation
    export BASE_DOMAIN=ratel.foundation

    # 데이터베이스 설정
    export DATABASE_TYPE=postgres
    export DATABASE_URL=postgresql://$(whoami)@localhost:5432/ratel_dev
    export MIGRATE=true

    # JWT 설정
    export JWT_SECRET_KEY=dev-jwt-secret-key-for-local-development-12345
    export AUTH_TYPE=jwt
    export JWT_EXPIRATION=3600

    # AWS 설정 (로컬 개발용 더미 값)
    export AWS_REGION=us-east-1
    export AWS_ACCESS_KEY_ID=dev-access-key
    export AWS_SECRET_ACCESS_KEY=dev-secret-key

    # API 키 설정
    export OPENAPI_KEY=dev-openapi-key
    export SECRET_TOKEN=dev-secret-token

    # S3 버킷 설정
    export BUCKET_NAME=dev-bucket
    export ASSET_DIR=metadata
    export BUCKET_EXPIRE=3600

    # Slack 채널 설정
    export SLACK_CHANNEL_SPONSOR=dev-sponsor
    export SLACK_CHANNEL_ABUSING=dev-abusing
    export SLACK_CHANNEL_MONITOR=dev-monitor

    # Firebase 설정
    export FIREBASE_API_KEY=dev-firebase-api-key
    export FIREBASE_AUTH_DOMAIN=dev.firebase.com
    export FIREBASE_PROJECT_ID=dev-project
    export FIREBASE_STORAGE_BUCKET=dev-bucket
    export FIREBASE_MESSAGING_SENDER_ID=123456789
    export FIREBASE_APP_ID=dev-app-id
    export FIREBASE_MEASUREMENT_ID=dev-measurement-id

    # API 엔드포인트
    export MAIN_API_ENDPOINT=http://localhost:3000
    export REDIRECT_URI=http://localhost:8080

    # 로그 레벨
    export RUST_LOG=debug

    # 실험 기능
    export EXPERIMENT=true

    # LLVM clang 설정 (WASM 컴파일용)
    export PATH="/opt/homebrew/opt/llvm/bin:$PATH"
    export CC=/opt/homebrew/opt/llvm/bin/clang
    export CXX=/opt/homebrew/opt/llvm/bin/clang++

    # PostgreSQL 경로 추가
    export PATH="/opt/homebrew/opt/postgresql@14/bin:$PATH"
}

# 메인 실행
main() {
    check_prerequisites
    setup_database
    setup_environment
    
    echo ""
    echo -e "${GREEN}✅ 환경변수 설정 완료!${NC}"
    echo -e "${GREEN}✅ Environment setup completed!${NC}"
    echo ""
    echo -e "${YELLOW}📝 사용법 / Usage:${NC}"
    echo "   source scripts/setup-local-env.sh"
    echo ""
    echo -e "${YELLOW}🚀 서버 시작 / Start servers:${NC}"
    echo "   # 터미널 1 / Terminal 1 (Backend):"
    echo "   cd packages/main-api && cargo run"
    echo ""
    echo "   # 터미널 2 / Terminal 2 (Frontend):"
    echo "   cd packages/main-ui && dx serve --fullstack"
    echo ""
    echo -e "${YELLOW}🌐 접속 주소 / Access URLs:${NC}"
    echo "   Frontend: http://localhost:8080"
    echo "   Backend:  http://localhost:3000"
    echo ""
    echo -e "${YELLOW}💡 팁 / Tips:${NC}"
    echo "   - 환경변수는 각 터미널에서 다시 설정해야 합니다."
    echo "   - Environment variables need to be set in each terminal."
    echo "   - PostgreSQL 서비스가 실행 중인지 확인하세요."
    echo "   - Make sure PostgreSQL service is running."
}

# 스크립트가 직접 실행될 때만 main 함수 호출
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
else
    # source로 실행될 때는 환경변수만 설정
    setup_environment
    echo -e "${GREEN}✅ 환경변수가 현재 셸에 설정되었습니다.${NC}"
    echo -e "${GREEN}✅ Environment variables set in current shell.${NC}"
fi 