#! /bin/bash

export AWS_ACCESS_KEY_ID=test
export AWS_SECRET_ACCESS_KEY=test
export COMMIT=local
export MIGRATE=true

# work on web
cd ts-packages/web
ECR_NAME=web make docker.build

cd ../..

# work on main-api
cd packages/main-api
ECR_NAME=main-api RUST_FLAG="--features no-secret" RUSTFLAGS="-A warnings" make docker.build

cd ../..

# work on fetcher
cd packages/fetcher
ECR_NAME=fetcher RUST_FLAG="--features no-secret" RUSTFLAGS="-A warnings" make docker.build

cd ../..

# Setting up testing infra
docker-compose --profile testing up -d

echo "Waiting for services to be ready..."

# Wait for Main API (180 second timeout for container startup)
echo "🔄 Checking Main API..."
timeout=180
interval=10
elapsed=0
while [ $elapsed -lt $timeout ]; do
    if curl -f -s http://localhost:3000/version > /dev/null 2>&1; then
        echo "✅ Main API is responding (took ${elapsed}s)"
        break
    fi
    echo "⏳ Main API not ready yet... (${elapsed}/${timeout}s)"
    sleep $interval
    elapsed=$((elapsed + interval))
done

if [ $elapsed -ge $timeout ]; then
    echo "❌ Main API failed to respond with 200 status within ${timeout}s"
    echo "🔍 Checking Docker services status:"
    docker-compose -f docker-compose.test.yaml ps
    echo "🔍 Main API logs:"
    docker-compose -f docker-compose.test.yaml logs main-api --tail=50
    exit 1
fi

# Wait for Web frontend (180 second timeout)
echo "🔄 Checking Web frontend..."
elapsed=0
while [ $elapsed -lt $timeout ]; do
    if curl -f -s http://localhost:8080/api/version > /dev/null 2>&1; then
        echo "✅ Web frontend is responding (took ${elapsed}s)"
        break
    fi
    echo "⏳ Web frontend not ready yet... (${elapsed}/${timeout}s)"
    sleep $interval
    elapsed=$((elapsed + interval))
done

if [ $elapsed -ge $timeout ]; then
    echo "❌ Web frontend failed to respond with 200 status within ${timeout}s"
    echo "🔍 Checking Docker services status:"
    docker-compose -f docker-compose.test.yaml ps
    echo "🔍 Web logs:"
    docker-compose -f docker-compose.test.yaml logs web --tail=50
    exit 1
fi

echo "🎉 All services are ready!"


# Run Playwright tests
mkdir -p test-results
make test

