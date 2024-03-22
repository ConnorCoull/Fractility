# Readme

This application is used to display fractals in a SPA.

.github features the workflows file for CI.

code-files contains:
* the rust-backend folder, feauting lib.rs which contains the Rust code, and cargo.toml which contains the "settings" for Rust code.

* src which contains the vue application, and necessarcy HTML, CSS, and JavaScript components.

* serve.sh and serve.bat used to run the application.

* additional supporting files.

and non-code files contains other .txt and .md files that helped in this project.

It is licensed under Mozilla Public License 2.0

## Build instructions

to build navigate into `code-files` and run `serve.sh` in linux terminals and `serve.bat` in Windows terminals, this will run the application on developer settings, for the build release, use `build.sh` or `build.bat`.

### Requirements

These are the versions of the tools that ran the application:
* Node.js v20.11.1
* npm v8.19.2
* vue/cli v5.0.8
* rustup v1.26.0
* wasm-pack v0.12.1

If vue-router needs to be installed use vue-router v4.3.0

### Build steps

If the requirements are installed run `serve.sh` or `serve.bat`, otherwise
* install node from `https://nodejs.org/en`
* see above for npm
* `npm install -g @vue/cli` to install vue
* install rustup from `https://www.rust-lang.org/tools/install`
* `cargo install wasm-pack` to install wasm-pack
* `npm install vue-router` to install vue-router

### Test steps

There is no need to test the application is working if the necessary requirements are installed, the lighthouse test in Chrome Developer tools was ran in evaluation.

