#!/bin/bash

# Ratel Web E2E Test Runner

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default values
BASE_URL="${PLAYWRIGHT_BASE_URL:-http://localhost:8080}"
HEADED="${HEADED:-false}"
DEBUG="${DEBUG:-false}"
PROJECT="${PROJECT:-}"
TEST_FILE="${TEST_FILE:-}"

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if service is running
check_service() {
    local url=$1
    local service_name=$2
    
    print_status "Checking if $service_name is running at $url..."
    
    if curl -s -f "$url" > /dev/null 2>&1; then
        print_status "$service_name is running ✓"
        return 0
    else
        print_warning "$service_name is not running at $url"
        return 1
    fi
}

# Function to wait for service
wait_for_service() {
    local url=$1
    local service_name=$2
    local max_attempts=${3:-30}
    local attempt=1
    
    print_status "Waiting for $service_name to be ready..."
    
    while [ $attempt -le $max_attempts ]; do
        if check_service "$url" "$service_name"; then
            return 0
        fi
        
        print_status "Attempt $attempt/$max_attempts - waiting 2 seconds..."
        sleep 2
        ((attempt++))
    done
    
    print_error "$service_name is not responding after $max_attempts attempts"
    return 1
}

# Function to install dependencies
install_deps() {
    print_status "Installing dependencies..."
    
    if [ ! -f "package.json" ]; then
        print_error "package.json not found. Make sure you're in the web package directory."
        exit 1
    fi
    
    npm install
    
    # Install Playwright browsers if needed
    if ! npx playwright --version > /dev/null 2>&1; then
        print_status "Installing Playwright browsers..."
        npx playwright install
    fi
}

# Function to run tests
run_tests() {
    local test_cmd="npx playwright test"
    
    # Add project filter if specified
    if [ -n "$PROJECT" ]; then
        test_cmd="$test_cmd --project=$PROJECT"
    fi
    
    # Add test file if specified
    if [ -n "$TEST_FILE" ]; then
        test_cmd="$test_cmd $TEST_FILE"
    fi
    
    # Add headed mode if requested
    if [ "$HEADED" = "true" ]; then
        test_cmd="$test_cmd --headed"
    fi
    
    # Add debug mode if requested
    if [ "$DEBUG" = "true" ]; then
        test_cmd="$test_cmd --debug"
    fi
    
    print_status "Running command: $test_cmd"
    eval $test_cmd
}

# Function to show usage
usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  -u, --url URL          Base URL for testing (default: http://localhost:8080)"
    echo "  -h, --headed           Run tests in headed mode"
    echo "  -d, --debug            Run tests in debug mode"
    echo "  -p, --project PROJECT  Run tests for specific project (chromium, firefox, etc.)"
    echo "  -t, --test TEST_FILE   Run specific test file"
    echo "  --no-wait             Skip waiting for services to be ready"
    echo "  --install-only        Only install dependencies, don't run tests"
    echo "  --report              Show test report"
    echo "  --help                Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0                                    # Run all tests"
    echo "  $0 --headed                          # Run tests with browser UI"
    echo "  $0 --project chromium                # Run tests only in Chrome"
    echo "  $0 --test auth.spec.ts               # Run only auth tests"
    echo "  $0 --debug --test navigation.spec.ts # Debug navigation tests"
    echo "  $0 --url http://staging.ratel.foundation # Test against staging"
}

# Main execution
main() {
    local wait_for_services=true
    local install_only=false
    local show_report=false
    
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -u|--url)
                BASE_URL="$2"
                shift 2
                ;;
            -h|--headed)
                HEADED=true
                shift
                ;;
            -d|--debug)
                DEBUG=true
                shift
                ;;
            -p|--project)
                PROJECT="$2"
                shift 2
                ;;
            -t|--test)
                TEST_FILE="$2"
                shift 2
                ;;
            --no-wait)
                wait_for_services=false
                shift
                ;;
            --install-only)
                install_only=true
                shift
                ;;
            --report)
                show_report=true
                shift
                ;;
            --help)
                usage
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                usage
                exit 1
                ;;
        esac
    done
    
    print_status "Ratel Web E2E Test Runner"
    print_status "Base URL: $BASE_URL"
    
    # Install dependencies
    install_deps
    
    if [ "$install_only" = "true" ]; then
        print_status "Dependencies installed. Exiting."
        exit 0
    fi
    
    if [ "$show_report" = "true" ]; then
        print_status "Opening test report..."
        npx playwright show-report
        exit 0
    fi
    
    # Wait for services if requested
    if [ "$wait_for_services" = "true" ]; then
        wait_for_service "$BASE_URL" "Web Application" || {
            print_error "Web application is not running. Start it first or use --no-wait flag."
            print_status "To start the application:"
            print_status "  npm run dev                    # For local development"
            print_status "  docker-compose up web          # Using Docker"
            exit 1
        }
        
        # Check API service
        api_url=$(echo "$BASE_URL" | sed 's/:8080/:3000/')
        if ! check_service "$api_url/version" "API Service"; then
            print_warning "API service may not be running, but continuing with tests..."
        fi
    fi
    
    # Run tests
    print_status "Starting E2E tests..."
    export PLAYWRIGHT_BASE_URL="$BASE_URL"
    
    if run_tests; then
        print_status "Tests completed successfully! ✓"
        
        # Show report if in interactive mode
        if [ "$HEADED" = "true" ] || [ "$DEBUG" = "true" ]; then
            print_status "Opening test report..."
            npx playwright show-report --open
        fi
    else
        print_error "Some tests failed ✗"
        print_status "Run with --report to see detailed results"
        exit 1
    fi
}

# Run main function with all arguments
main "$@"