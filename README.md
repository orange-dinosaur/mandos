# MANDOS

gRPC server for user Authentication, Authorization and Session Management written in ```rust```.

## Dev Setup

Command to run the server:

```bash
source .env && cargo watch -q -c -w src/ -x "run --release --bin mandos"
```

Inside the ```root``` directory there as to be an ```.env``` file that will be used to load environment variables for the development.
The env variables are loades before running the server using the ```source``` command
The file has to contain the following variables:

```bash
# Environment
# Possible values: test, development, production
# Optional (default: development)
export ENVIRONMENT="development"

# gRPC Server
export GRPC_AUTH_KEY="key"
export GRPC_AUTH_VALUE="secret"

# Database (PostgreSQL)
export DB_USER="db_user"
export DB_PASSWORD="db_password"
export DB_HOST="db_hostname"
export DB_PORT="0000"
export DB_NAME="db_name"

# Session Database (Redis)
export SESSION_DB_USER="session_db_user"
export SESSION_DB_PASSWORD="session_db_password"
export SESSION_DB_HOST="session_db_hostname"
export SESSION_DB_PORT="0000"
```

## Test Setup

Command to run the tests:

```bash
cargo test
```

Inside the ```root``` directory there as to be an ```.tests.env``` file that will be used to load environment variables for the tests.
The file has to contain the following variables:

```bash
export ENVIRONMENT="test"

# gRPC Server
export GRPC_AUTH_KEY="key"
export GRPC_AUTH_VALUE="secret"

# Database (PostgreSQL)
export DB_USER="db_user"
export DB_PASSWORD="db_password"
export DB_HOST="db_hostname"
export DB_PORT="0000"
export DB_NAME="db_name"

# Session Database (Redis)
export SESSION_DB_USER="session_db_user"
export SESSION_DB_PASSWORD="session_db_password"
export SESSION_DB_HOST="session_db_hostname"
export SESSION_DB_PORT="0000"
```