# MANDOS

gRPC server for Authentication, Authorization and Session Management written in ```rust```.

## Dev Setup

Command to run the server:

```bash
cargo watch -q -c -w src/ -x "run --release --bin server"
```

## Env variables

The ```.env``` files contains the variables:

```bash
# gRPC Server
export SERVICE_GRPC_AUTH_VALUE="secret"

# Database
export SERVICE_DB_USER_TEST="db_user"
export SERVICE_DB_PASSWORD_TEST="db_password"
export SERVICE_DB_HOST_TEST="db_hostname"
export SERVICE_DB_PORT_TEST="0000"
export SERVICE_DB_NAME_TEST="db_name"

export SERVICE_DB_USER_DEV="db_user"
export SERVICE_DB_PASSWORD_DEV="db_password"
export SERVICE_DB_HOST_DEV="db_hostname"
export SERVICE_DB_PORT_DEV="0000"
export SERVICE_DB_NAME_DEV="db_name"
```