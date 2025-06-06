# 🚀 Ratel Local Development Guide

Complete guide for setting up and running Ratel in your local development environment.

## 📋 Table of Contents

- [🎯 Overview](#-overview)
- [⚡ Quick Start](#-quick-start)
  - [1. Prerequisites](#1-prerequisites)
  - [2. Clone and Start](#2-clone-and-start)
  - [3. Access the Application](#3-access-the-application)
- [🔥 Firebase Emulator Features](#-firebase-emulator-features)
- [📁 Project Structure](#-project-structure)
- [🛠️ Manual Setup (Alternative)](#️-manual-setup-alternative)
- [🔧 Troubleshooting](#-troubleshooting)
- [📊 Development Commands](#-development-commands)
- [🚀 Advanced Configuration](#-advanced-configuration)
- [❓ FAQ](#-faq)

## 🎯 Overview

Ratel uses an integrated development environment with:
- **Backend**: Rust + Axum API server
- **Frontend**: Dioxus (React-like for Rust) 
- **Database**: PostgreSQL
- **Authentication**: Firebase Emulator (for local testing)
- **Build Tools**: Cargo, Dioxus CLI, Tailwind CSS

Using Firebase emulator, you can test all features including Google login without needing an actual Firebase project.

## ⚡ Quick Start

### 1. Prerequisites

Install required tools:

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Dioxus CLI
cargo install cargo-binstall && cargo binstall dioxus-cli

# Firebase CLI
npm install -g firebase-tools

# Java (Required for Firebase Emulator)
brew install openjdk@11
echo 'export PATH="/opt/homebrew/opt/openjdk@11/bin:$PATH"' >> ~/.zshrc

# PostgreSQL (macOS)
brew install postgresql@14 && brew services start postgresql@14

# Node.js (for Firebase CLI)
brew install node
```

### 2. Clone and Start

```bash
git clone https://github.com/biyard/ratel.git
cd ratel
make start
```

That's it! The Makefile will automatically:
- ✅ Check and install missing dependencies
- ✅ Set up environment variables  
- ✅ Start PostgreSQL database
- ✅ Launch Firebase emulator
- ✅ Start backend and frontend servers
- ✅ Monitor all services with real-time logs

### 3. Access the Application

Once all services are running, you can access:

- **Frontend**: [http://localhost:8080](http://localhost:8080) (Main web application)
- **Backend API**: [http://localhost:3000](http://localhost:3000) (REST API server)
- **Firebase UI**: [http://localhost:4000](http://localhost:4000)
- **Firebase Auth Emulator**: [http://localhost:9099](http://localhost:9099)

## 🔥 Firebase Emulator Features

The Firebase emulator provides:

1. **Local Google Authentication** - No real Google account needed
2. The Firebase emulator will show a fake Google login dialog
3. **User Management** - Create and manage test users
4. **Real-time Database** - Local Firestore instance

### Testing Authentication

1. Open Firebase UI: [http://localhost:4000](http://localhost:4000)
2. Go to Authentication tab
3. Add test users or use the fake Google login

## 📁 Project Structure

```text
ratel/
├── packages/
│   ├── main-ui/             # Dioxus frontend
│   ├── main-api/            # Axum backend
│   └── dto/                 # Shared data types
├── scripts/
├── firebase.json             # Firebase emulator config
├── .firebaserc              # Firebase project config
├── Makefile                 # Development workflow
├── logs/
│   ├── backend.log
│   ├── frontend.log
│   └── firebase.log
└── docs/                    # Documentation
```

## 🛠️ Manual Setup (Alternative)

If you prefer manual control over each service:

### Environment Setup
```bash
make setup
```

### Individual Services
```bash
# Start individual services
make dev        # Frontend only (fast development)
make run        # Frontend only (with clean)

# Start full environment
make start

# Stop all services  
make stop
```

### Manual Database Setup
```bash
# Start PostgreSQL
brew services start postgresql@14

# Create database
createdb ratel_dev

# Test connection
psql ratel_dev -c "SELECT 1;"
```

## 🔧 Troubleshooting

### Common Issues

#### 1. Firebase Emulator Won't Start

**Error**: Java not found
```bash
# Install Java
brew install openjdk@11

# Add to PATH
echo 'export PATH="/opt/homebrew/opt/openjdk@11/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc

# Then run setup
make setup
```

**Error**: Port conflicts
```bash
# Kill existing Firebase processes
pkill -f "firebase.*emulators"

# Check Firebase emulator logs
cat logs/firebase.log
```

#### 2. Backend Server Issues

**Error**: Environment variables not set
- Environment variables are automatically set by the Makefile
- Run `make setup` to verify prerequisites

**Error**: Database connection failed
```bash
# Start PostgreSQL
brew services start postgresql@14

# Create database
createdb ratel_dev
```

**Error**: Port 3000 already in use
```bash
lsof -i :3000
kill -9 <PID>
```

#### 3. Frontend Compilation Issues

**Error**: Dioxus CLI not found
```bash
cargo install dioxus-cli
```

**Error**: WASM compilation failed
```bash
export CC=/opt/homebrew/opt/llvm/bin/clang
```

**Error**: Missing web features
- Ensure Firebase emulator is running on port 9099
- Check environment variables are set
- Verify Firebase emulator configuration in browser dev tools

## 📊 Monitoring and Logs

### Real-time Log Monitoring
```bash
# All logs
tail -f logs/*.log

# Individual services
tail -f logs/backend.log     # Backend server
tail -f logs/frontend.log    # Frontend server
# Firebase emulator
tail -f logs/firebase.log
```

### Service Status
```bash
# Check if services are running
curl http://localhost:3000/health  # Backend
curl http://localhost:8080         # Frontend
curl http://localhost:9099         # Firebase Auth
curl http://localhost:4000         # Firebase UI
```

## ⚙️ Configuration

### Environment Variables

The Makefile automatically sets these variables when running development targets:

#### Firebase Configuration
```bash
FIREBASE_API_KEY="fake-api-key-for-emulator"
FIREBASE_AUTH_DOMAIN="localhost"
FIREBASE_PROJECT_ID="ratel-local-dev"
FIREBASE_STORAGE_BUCKET="ratel-local-dev.appspot.com"
FIREBASE_MESSAGING_SENDER_ID="123456789"
FIREBASE_APP_ID="1:123456789:web:fake-app-id"
FIREBASE_MEASUREMENT_ID="G-FAKE-ID"
```

#### Emulator Hosts
```bash
FIREBASE_AUTH_EMULATOR_HOST="localhost:9099"
FIRESTORE_EMULATOR_HOST="localhost:8081"
FIREBASE_EMULATOR_HUB="localhost:4400"
```

#### Database Configuration
```bash
DATABASE_URL="postgres://$(whoami)@localhost:5432/ratel_dev"
DATABASE_TYPE="postgres"
```

### Custom Configuration

You can override any environment variable by setting it before running make targets:

```bash
export DATABASE_URL="postgres://myuser:mypass@localhost:5432/my_ratel_db"
make start
```

## 🧪 Testing

### Authentication Testing

1. **Start the development environment**:
   ```bash
   make start
   ```

2. **Open the application**: [http://localhost:8080](http://localhost:8080)

3. **Test login flow**:
   - Click "Login with Google"
   - Firebase emulator will show a fake login dialog
   - Enter any email (e.g., test@example.com)
   - Complete the authentication flow

4. **Use the Firebase emulator login**:
   - The emulator provides fake Google authentication
   - No real Google account required
   - Users are stored in the local emulator

5. **Verify in Firebase UI**:
   - Open [http://localhost:4000](http://localhost:4000)
   - Go to Authentication tab
   - Firebase UI should show the logged-in user

### API Testing

```bash
# Test backend health
curl [http://localhost:3000/health](http://localhost:3000/health)

# Test API endpoints
curl [http://localhost:3000/api/v1/users](http://localhost:3000/api/v1/users)
```

## 🚀 Development Workflow

### Recommended Workflow

1. **Start development environment**:
   ```bash
   make start
   ```

2. **Keep Firebase UI open** to manage test users

3. **Use hot reload** for frontend development

4. **Use browser dev tools** to inspect Firebase configuration

5. **Monitor logs** for debugging:
   ```bash
   tail -f logs/backend.log logs/frontend.log
   ```

### Stopping Services

```bash
# Stop all services
make stop

# Or use Ctrl+C if running in foreground
```

### Cleaning Up

```bash
# Clean logs and temporary files
make clean-dev

# Check service status
make status
```

## 📚 Additional Resources

- [Dioxus Documentation](https://dioxuslabs.com/)
- [Axum Documentation](https://docs.rs/axum/)
- [Firebase Emulator Suite](https://firebase.google.com/docs/emulator-suite)
- [PostgreSQL Documentation](https://www.postgresql.org/docs/)

## 🤝 Getting Help

If you encounter issues:

1. **Check the logs**: `cat logs/[service].log`
2. **Verify tool installation**: `make status`
3. **Clean and restart**: `make clean-dev && make start`
4. **Check this guide** for common solutions
5. **Open an issue** on GitHub with error logs

---

**Happy Developing! 🎉** 