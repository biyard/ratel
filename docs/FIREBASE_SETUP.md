# Firebase Environment Setup Guide

This document provides step-by-step instructions for setting up Firebase environments for the Ratel project.

## Overview

The Ratel web application uses Firebase for authentication services, specifically Google OAuth authentication with Google Drive integration for key pair management. Firebase is configured through environment variables and supports multiple deployment environments (local, dev, staging, prod).

## Prerequisites

- Firebase project created in [Firebase Console](https://console.firebase.google.com/)
- Google Drive API enabled for the Firebase project
- Node.js and npm/pnpm installed
- Access to the Ratel repository

## Firebase Project Setup

### 1. Create Firebase Project

1. Go to [Firebase Console](https://console.firebase.google.com/)
2. Click "Create a project"
3. Enter project name (e.g., `ratel-local`)
4. Enable Google Analytics (optional)
5. Complete project creation

### 2. Configure Authentication

1. In Firebase Console, go to **Authentication** > **Sign-in method**
2. Enable **Google** as a sign-in provider
3. Add authorized domains:
   - For development: `localhost`
   - For staging: `stg.ratel.foundation`
   - For production: `ratel.foundation`

### 3. Enable Google Drive API

1. Go to [Google Cloud Console](https://console.cloud.google.com/)
2. Select your Firebase project
3. Navigate to **APIs & Services** > **Library**
4. Search for "Google Drive API" and enable it
5. Configure OAuth consent screen if prompted

### 4. Get Firebase Configuration

1. In Firebase Console, go to **Project Settings** > **General**
2. Under "Your apps", click "Web app" (</> icon)
3. Register your app with appropriate name
4. Copy the Firebase configuration object

## Environment Configuration

### Required Environment Variables

The following environment variables must be set for Firebase integration:

```bash
# Firebase Configuration
FIREBASE_API_KEY=AIzaSy...
FIREBASE_AUTH_DOMAIN=your-project.firebaseapp.com
FIREBASE_PROJECT_ID=your-project-id
FIREBASE_STORAGE_BUCKET=your-project.appspot.com
FIREBASE_MESSAGING_SENDER_ID=123456789
FIREBASE_APP_ID=1:123456789:web:abcdef
FIREBASE_MEASUREMENT_ID=G-XXXXXXXXXX  # Optional, for Google Analytics
```

### Environment-Specific Setup

#### Local Development

1. **Using Docker Compose (Recommended)**:
   ```bash
   # Set Firebase environment variables in your shell or .env file
   export NEXT_PUBLIC_FIREBASE_API_KEY=AIzaSy...
   export NEXT_PUBLIC_FIREBASE_AUTH_DOMAIN=your-project.firebaseapp.com
   # ... other variables

   # Start development environment
   docker-compose --profile development up -d
   ```

2. **Manual Setup**:
   ```bash
   cd ts-packages/web

   # Set environment variables
   export FIREBASE_API_KEY=AIzaSy...
   export FIREBASE_AUTH_DOMAIN=your-project.firebaseapp.com
   # ... other Firebase variables

   # Generate .env.local file
   ./setup-env.sh

   # Start development server
   make run
   ```

#### Production/Staging Deployment

For production deployments, set the environment variables in your deployment platform:

```bash
# Example for Docker build
docker build \
  --build-arg NEXT_PUBLIC_FIREBASE_API_KEY=$FIREBASE_API_KEY \
  --build-arg NEXT_PUBLIC_FIREBASE_AUTH_DOMAIN=$FIREBASE_AUTH_DOMAIN \
  --build-arg NEXT_PUBLIC_FIREBASE_PROJECT_ID=$FIREBASE_PROJECT_ID \
  --build-arg NEXT_PUBLIC_FIREBASE_STORAGE_BUCKET=$FIREBASE_STORAGE_BUCKET \
  --build-arg NEXT_PUBLIC_FIREBASE_MESSAGING_SENDER_ID=$FIREBASE_MESSAGING_SENDER_ID \
  --build-arg NEXT_PUBLIC_FIREBASE_APP_ID=$FIREBASE_APP_ID \
  --build-arg NEXT_PUBLIC_FIREBASE_MEASUREMENT_ID=$FIREBASE_MEASUREMENT_ID \
  -t ratel-web .
```

## Configuration Files

### Automatic Environment Setup

The project includes `ts-packages/web/setup-env.sh` which automatically creates `.env.local` from environment variables:

```bash
cd ts-packages/web
./setup-env.sh
```

This script:
- Checks if `.env.local` already exists
- Creates `.env.local` with appropriate `NEXT_PUBLIC_*` prefixed variables
- Configures development/production API endpoints

### Manual .env.local Configuration

If you prefer manual setup, create `ts-packages/web/.env.local`:

```bash
# Environment
NEXT_PUBLIC_ENV=dev

# Firebase Configuration
NEXT_PUBLIC_FIREBASE_API_KEY=AIzaSy...
NEXT_PUBLIC_FIREBASE_AUTH_DOMAIN=your-project.firebaseapp.com
NEXT_PUBLIC_FIREBASE_PROJECT_ID=your-project-id
NEXT_PUBLIC_FIREBASE_STORAGE_BUCKET=your-project.appspot.com
NEXT_PUBLIC_FIREBASE_MESSAGING_SENDER_ID=123456789
NEXT_PUBLIC_FIREBASE_APP_ID=1:123456789:web:abcdef
NEXT_PUBLIC_FIREBASE_MEASUREMENT_ID=G-XXXXXXXXXX

# API Configuration
NEXT_PUBLIC_ENABLE_SERVER_CONFIG=false
NEXT_PUBLIC_API_URL=http://localhost:3000
NEXT_PUBLIC_EXPERIMENT=false
```

## Validation and Testing

### Firebase Configuration Validation

The application automatically validates Firebase configuration on startup:

- Checks all required fields are present and non-empty
- Validates API key format (should start with "AIza")
- Validates project ID format (alphanumeric and hyphens only)
- Logs warnings for invalid configurations

### Testing Firebase Integration

1. **Start the application**:
   ```bash
   cd ts-packages/web
   npm run dev
   ```

2. **Check browser console** for Firebase initialization messages:
   - ✅ `🔥 Firebase initialized successfully`
   - ❌ `🔥 Firebase configuration is invalid or incomplete`
   - ❌ `🔥 Failed to initialize Firebase`

3. **Test Google authentication**:
   - Open the application
   - Attempt to sign in with Google
   - Verify successful authentication and key pair management

## Troubleshooting

### Common Issues

1. **"Firebase configuration is invalid or incomplete"**
   - Verify all required environment variables are set
   - Check for typos in variable names
   - Ensure values are not empty, "undefined", or "null"

2. **"Firebase API key appears to be invalid format"**
   - Firebase API keys should start with "AIzaSy"
   - Verify you copied the correct API key from Firebase Console

3. **"Firebase project ID contains invalid characters"**
   - Project IDs should only contain lowercase letters, numbers, and hyphens
   - Verify the project ID matches exactly from Firebase Console

4. **Google authentication popup blocked**
   - Check if popup blockers are interfering
   - Ensure authorized domains are configured in Firebase Authentication

5. **Google Drive API errors**
   - Verify Google Drive API is enabled in Google Cloud Console
   - Check OAuth consent screen configuration
   - Ensure proper scopes are requested (`https://www.googleapis.com/auth/drive.appdata`)

### Debug Mode

Enable debug logging by setting:
```bash
NEXT_PUBLIC_LOG_LEVEL=debug
```

This will provide detailed Firebase initialization and authentication logs.

## Security Considerations

- **Never commit Firebase credentials to version control**
- Use environment variables for all Firebase configuration
- Restrict Firebase project access to authorized domains only
- Regularly rotate Firebase API keys if compromised
- Monitor Firebase usage and authentication logs

## Multiple Environment Setup

For different environments (dev, staging, prod), create separate Firebase projects:

1. **Development**: `ratel-dev`
   - Domain: `dev.ratel.foundation`
   - Less restrictive for testing

2. **Staging**: `ratel-staging`
   - Domain: `stg.ratel.foundation`
   - Production-like configuration

3. **Production**: `ratel-prod`
   - Domain: `ratel.foundation`
   - Strict security settings

Each environment should have its own set of Firebase credentials and be configured with appropriate authorized domains and API restrictions.
