[English](README.md)|[日本語](README-ja.md)

<h1 align="center">migrate</h1>

<h2 align="center">A Simple Migration App</h2>

This is a simple migration application that can be executed in the console, using PostgreSQL as the database.

# Prerequisites

- This application assumes the use of PostgreSQL.
- Database settings are managed via a .env file.
- The application is not distributed as binary files, so you need to build it in your environment.

# Environment Setup

## How to Build

Execute the following in an environment where Rust is installed.

```shell
cargo build --release
```

Place the built executable file in your directory.

The built file will be located at the following path:

```shell
./target/release/migrate
```

## Database Access Configuration

You will need to configure the database access information.

Follow the steps below:

1. Place a .env file in the same directory as the built migration application.
2. Write the database access settings in the .env file in the following format:

```env
DATABASE_URL=postgres://username:password@hostname:port/db_name
```

# Preliminary Steps

## Creating a Table for Migration Management

First, create a table to manage the migrations.

Ensure you have a PostgreSQL environment and have set up the connection in the `.env` file before executing.

```shell
migrate -i
# or
migrate --init
```

After executing the command, a table named `migrations` will be created in the database for managing the migrations.

## Creating Migration Files

Create files to define the migrations you want to execute.

```shell
migrate -c
# or
migrate --create
```

After executing the command, files like the following will be created:

```shell
# up file
./Migrations/<YYYY-MM-DD>_<UNIX_TIME_STAMP>_up.sql
# ./Migrations/2000-01-01_1234567890_up.sql

# down file
./Migrations/<YYYY-MM-DD>_<UNIX_TIME_STAMP>_down.sql
# ./Migrations/2000-01-01_1234567890_down.sql
```

## Configuring the Migrations to Execute

Write the migrations you want to execute in the created `up file` and `down file`.

### up file

Write the migrations you want to execute.

Any statement that can be executed as SQL can be written.

It is also possible to write multiple queries.

Make sure to end each SQL statement with a `;`.

For example, you might write a SQL statement to create a table like this:

```sql
CREATE TABLE users (
                id BIGINT PRIMARY KEY AUTO_INCREMENT,
                username VARCHAR(255) NOT NULL,
                email VARCHAR(255) NOT NULL UNIQUE,
                password VARCHAR(255) NOT NULL,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
```

### down file

Write the SQL statement to rollback the results executed in the up file.

For example,

- If you define table creation in `up file`, define a SQL statement to delete that table in `down file`.
- If you define an insert statement in `up file`, define a SQL statement to delete the inserted data in `down file`.

In the example below, the SQL statement to delete the table defined in `up file` is written:

```sql
DROP TABLE users;
```

# Executing Migrations

After completing the environment setup and preliminary steps, the migrations are executed with the following command:

```shell
migrate
```

## Migration Target Files

Migrations that have been executed in the past are managed in the `migrations` table.

If you define new migration files, only the newly added migrations will be executed.

For example:

```shell
# Execute the migration
migrate

# Create a new migration file
migrate --create

# Add SQL statements to the migration file
# Omitted here

# Execute the new migration
# Only the newly added migration will be executed
migrate
```

# Rollback

It is possible to rollback the executed migrations to a specific stage.

The command is as follows:

```shell
# <n> specifies the number of stages to rollback
migrate -r <n>
# or
migrate --rollback <n>

# Example
# To rollback to two stages before
migrate -r 2
# or
migrate --rollback 2
```

Also, it will only execute the possible number of rollbacks.

For a database where two migrations have been performed, specifying a number greater than 2 will only perform two rollbacks (even if 10 or 1000 is specified).

# Help

If you are having trouble with the commands, refer to the help.

You can refer to the help with the following command:

```shell
migrate -h
# or
migrate --help
```

# LICENSE

[MIT LICENSE](https://github.com/kip2/sqcr/blob/main/LICENSE)

# AUTHOR

[kip2](https://github.com/kip2)