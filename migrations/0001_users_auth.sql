create table users_auth (
    id uuid PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    last_login TIMESTAMPTZ,
    needs_verify BOOLEAN NOT NULL,
    is_blocked BOOLEAN NOT NULL,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL
);

create unique index users_auth_username_idx on users_auth(username);
create unique index users_auth_email_idx on users_auth(email);