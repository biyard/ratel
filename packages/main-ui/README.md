# Dioxus Template For Web

This repository provides a template of dioxus.
It serves fullstack web server, but we recommend you not to add any server implementation to reduce building time.

Instead of it, you use cargo workspace and separated API packages.
- And use reqwest to call server APIs.
- For SSR, you can use `use_future_server` in web code.

## Running a web server

``` bash
make run
```
