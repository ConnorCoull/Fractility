#!/bin/bash
cd ./rust-backend/
cargo install wasm-pack
npm install
wasm-pack build --target web
cd ..
rm -rf public/pkg
cp -r rust-backend/pkg/ public/pkg
npm run build