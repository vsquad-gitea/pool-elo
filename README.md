# Installing requirements

## 1. Install rust:
### Windows:

Download installer from https://www.rust-lang.org/tools/install

### Unix based systems:

Run `curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh`

## 2. Install npm

### Windows:

https://nodejs.org/en
(todo look into:)
https://pnpm.io/

### Unix based systems:

`sudo apt install nodejs`
`sudo apt install npm`

## 3. Install Perseus, for real-time updates while developing

`cargo install perseus-cli`

(temporarily broken, if this doensn't work run `cargo install perseus-cli --locked` )

`rustup target add wasm32-unknown-unknown`

## 4. Install docker for Postgresql

## 5. Install SeaORM for database

`cargo install sea-orm-cli`

## 5. Install tailwindcss, for styling

`npm install -D tailwindcss`

Also take a look at

Website:
https://framesurge.sh/perseus/en-US/

Simple tutorial:
https://blog.logrocket.com/building-rust-app-perseus/

# Building the project

To set up the database, run:
`$env:DATABASE_URL = "postgres://elo:elo@localhost:5432/elo_app"; sea-orm-cli migrate up`

Updating entities after updating database:
`$env:DATABASE_URL = "postgres://elo:elo@localhost:5432/elo_app"; sea-orm-cli generate entity -o entity/src --with-serde both`

To build CSS run:
`npm run build`

To build the project for testing, run
`perseus serve --verbose`
(if broken: todo remove once fixed)
`perseus --wasm-opt-version version_118 serve --verbose`

# Deploying the project

First run
`perseus deploy`

The folder with everything necessary will be in `/pkg`
