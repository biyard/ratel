# DemocraSee

## Development
### Running API Server
- It runs =SERVICE= crate.
  - Default =SERVICE= is =main-ui=.

``` bash
export SERVICE=main-api
make run
```

### Running Web UI
- It will interact with API server in =dev= environment.
  - If you want to change it, set =API_ENDPOINT= environment.

``` bash
export SERVICE=main-ui
export API_ENDPOINT=http://localhost:3000

make run
```


