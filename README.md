# Ntex Rust

This repository provides of:

- ✅ Ntex REST API
- ✅ Error handling
- ✅ JWT authentication
- ✅ Interaction with the MySql database
- ✅ Password encryption
- ✅ Payload validation
- ✅ Ntex CORS config
- 🚫 Ntex Swagger (OpenAPI)

## Required

- Rust

## Usage

- edit .env or .env.prod for production mode
- `cargo run --release` or `debug with vscode`

## Docker build

- `docker buildx build . -t youraccount/ntex-web-api  --platform linux/amd64 --push`

## Roadmap next version ?

- Swagger OpenAPI support (utoipa)
- Cancellation token
- Tokio support
- Lazy static
- Image optimize and reducer
- Clap environment config

`Thanks for review my code...`
