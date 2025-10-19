# Custom Backend Services Orchestrator CLI Tool

A simple custom command-line interface (CLI) tool for interacting with a
PostgreSQL database, built in Rust.

## Overview

This CLI lets users execute common database operations such as
retrieving, inserting, listing, or deleting user logs.\
One of its key commands, `get`, retrieves the **first 10 rows** from the
`ops.client_connect_log` table and prints them in a formatted style.

## Features

- Connects securely to a PostgreSQL database using connection details from `.env`
- Supports multiple subcommands:
  - `get` --- retrieve first 10 logs
  - `list`, `set`, `delete`, `search`, `help`, `exit`
- Provides interactive mode when no command argument is given
- Offers auto-completion suggestions for command names
- Handles nullable database fields safely using Rust's `Option` types
- Pretty colored terminal output via [`colored`](https://docs.rs/colored/)

## Installation

### 1. Prerequisites

Ensure you have: - **Rust toolchain** (via
[rustup](https://rustup.rs)) - **PostgreSQL** running and accessible - A
valid `.env` file in your project root containing:
`bash   DATABASE_URL=postgres://user:password@localhost:5432/your_db`

### 2. Build

``` bash
cargo build --release
```

### 3. Run

To start the CLI:

``` bash
cargo run -- get
```

Or interactively:

``` bash
cargo run
```

## Example Usage

``` bash
$ cargo run -- get
Executing get command...
Values from DB:
  1. id=1 | occurred_at=2025-10-19T14:00:00Z | user_id=42 | username=alice | ip=192.168.1.10 | session_id=550e8400-e29b-41d4-a716-446655440000 | ua=Mozilla/5.0
  2. id=2 | occurred_at=2025-10-19T14:02:00Z | user_id=43 | username=bob   | ip=192.168.1.11 | session_id=550e8400-e29b-41d4-a716-446655440001 | ua=Mozilla/5.0
  ...
```

When no `cmd` argument is given, the tool prompts:

    Enter Command: 

with autocomplete suggestions like `get`, `set`, `delete`, etc.
