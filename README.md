# DemocraSee

## Common Environments
| Name     | Description                                                  |
|----------|--------------------------------------------------------------|
| RUST_LOG | Logging libraries based on tracing(`debug`, `info`, `error`) |


## Development
### Running API Server
- It runs =SERVICE= crate.
  - Default =SERVICE= is =main-ui=.

``` bash
export SERVICE=main-api
export OPENAPI_KEY=`your openapi key`
make run
```

### Running Web UI
- It will interact with API server in `dev` environment.
  - If you want to change it, set `MAIN_API_ENDPOINT` environment.
- Before running UI, set up firebase configuration

``` bash
export SERVICE=main-ui
export MAIN_API_ENDPOINT=http://localhost:3000

export FIREBASE_API_KEY=""
export FIREBASE_AUTH_DOMAIN=""
export FIREBASE_PROJECT_ID=""
export FIREBASE_STORAGE_BUCKET=""
export FIREBASE_MESSAGING_SENDER_ID=""
export FIREBASE_APP_ID=""
export FIREBASE_MEASUREMENT_ID=""

make run
```


## Deployment
### Main API

``` bash
export SERVICE=main-api
export ENABLE_S3=false
export ENABLE_FARGATE=false
export ENABLE_DYNAMO=false
export ENABLE_RDS=true
export ENABLE_LAMBDA=true
export DOMAIN=api.dev.democrasee.me
export RDS_ADMIN_PASSWORD=""

ENV=dev make deploy
```
