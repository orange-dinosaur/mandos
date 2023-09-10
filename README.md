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
export GRPC_AUTH_VALUE="secret"

# Database
export DB_USER_TEST="db_user"
export DB_PASSWORD_TEST="db_password"
export DB_HOST_TEST="db_hostname"
export DB_PORT_TEST="0000"
export DB_NAME_TEST="db_name"

export DB_USER_DEV="db_user"
export DB_PASSWORD_DEV="db_password"
export DB_HOST_DEV="db_hostname"
export DB_PORT_DEV="0000"
export DB_NAME_DEV="db_name"
```