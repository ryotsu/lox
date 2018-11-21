## Lox

A lox interpreter written in Rust.

#### To run it locally:

 - Compile to `wasm` with `wasm-pack build`
 - Install dependencies in `www` with `npm install`
 - Link the contents of `./pkg` with `npm link` and `npm link lox`
 - Run development server `npm run start`
 - Browse `https://localhost:8080/`


#### Features:

 - [x] Scanner
 - [x] Parser
 - [x] Control Flow
 - [x] Functions
 - [x] Closures
 - [ ] Classes
 - [ ] Inheritence

 ---
 [Crafting Interpreters](http://www.craftinginterpreters.com)
