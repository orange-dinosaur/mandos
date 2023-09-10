# MANDOS

gRPC server for user Authentication, Authorization and Session Management written in ```rust```.

## Dev Setup

Command to run the server:

```bash
cargo watch -q -c -w src/ -x "run --release --bin server"
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
# Optional for test and development enviroments (default: 127.0.0.1)
export SERVER_URL="127.0.0.1"
# Optional for test and development enviroments (default: 50051)
export SERVER_PORT="0000"

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

If the environment is set to ```test``` or ```development``` the environment variables have to have postfix ```_TEST``` or ```_DEV``` (Except for `ENVIRONMENT` and `DB_MAX_CONNECTIONS`).

If for example the environment is set to ```development``` the environment variables will have to look like this:

```bash
# gRPC Server
export GRPC_AUTH_KEY_DEV="key"
export GRPC_AUTH_VALUE_DEV="secret"
export SERVER_URL_DEV="127.0.0.1"
export SERVER_PORT_DEV="0000"

# Database (PostgreSQL)
export DB_USER_DEV="db_user"
export DB_PASSWORD_DEV="db_password"
export DB_HOST_DEV="db_hostname"
export DB_PORT_DEV="0000"
export DB_NAME_DEV="db_name"

# Session Database (Redis)
export SESSION_DB_USER_DEV="session_db_user"
export SESSION_DB_PASSWORD_DEV="session_db_password"
export SESSION_DB_HOST_DEV="session_db_hostname"
export SESSION_DB_PORT_DEV="0000"
```