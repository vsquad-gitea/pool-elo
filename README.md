# Installing requirements

## 1. Install rust:
### Windows: 

Download installer from https://www.rust-lang.org/tools/install

### Unix based systems:

Run `curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh`

## 2. Install npm

### Windows: 

https://nodejs.org/en

### Unix based systems:

`sudo apt install nodejs`
`sudo apt install npm`

## 3. Install Perseus, for real-time updates while developing

`cargo install perseus-cli`
`cargo build --target wasm32-unknown-unknown`

## 4. Install tailwindcss, for styling

`npm install -D tailwindcss`

Also take a look at 

Website:
https://framesurge.sh/perseus/en-US/

Simple tutorial:
https://blog.logrocket.com/building-rust-app-perseus/

# Building the project

To build CSS run:
`npm run build`

To build the project for testing, run
`perseus serve`

# Deploying the project

First run
`perseus deploy`

The folder with everything necessary will be in `/pkg`
