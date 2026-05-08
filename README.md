# Mocking SpongeBob - WebAssembly Demo

This is an example project of integrating WebAssembly into a web app created with Vite. It was created simply for educational purposes.

## System Requirements

This project uses [Node.js](https://nodejs.org/en) and [Rust](https://rust-lang.org). Refer to the each website for installation instructions.

## Build Requirements

To generate the WebAssembly code from Rust code, a dedicated build tool is used: [wasm-pack](https://github.com/wasm-bindgen/wasm-pack).

After installing Rust, run the following command to install the **wasm-pack** CLI:

```
cargo install wasm-pack
```

## How to generate the Wasm module

Execute this step before running the web app. Inside the `wasm` directory run the following command:

```
wasm-pack build --target web
```

It should create a new `pkg` folder with the code for the Wasm module.

## Installing and running the web app

To install the npm dependencies run: 

```
npm install
```

To run the web app:

```
npm run dev
```

Go to http://localhost:5173/mocking-spongebob to see the running web app.