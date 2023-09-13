# MANDOS

gRPC server for user Authentication, Authorization and Session Management written in ```rust```.

## Dev Setup

Command to run the server:

```bash
cargo watch -q -c -w src/ -x "run --release --bin mandos"
```

## Env variables

The following environment variables are required to run the server:

```bash
# Environment
# Possible values: test, development, production
# Optional (default: development)
export ENVIRONMENT="production"

# gRPC Server
export GRPC_AUTH_KEY="key"
export GRPC_AUTH_VALUE="secret"

# Database (PostgreSQL)
export DB_USER="db_user"
export DB_PASSWORD="db_password"
export DB_HOST="db_hostname"
export DB_PORT="0000"
export DB_NAME="db_name"
# Optional (default: 5)
export DB_MAX_CONNECTIONS="5"

# Session Database (Redis)
export SESSION_DB_USER="session_db_user"
export SESSION_DB_PASSWORD="session_db_password"
export SESSION_DB_HOST="session_db_hostname"
export SESSION_DB_PORT="0000"
```