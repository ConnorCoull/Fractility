#!/bin/bash
cd ./rust-backend/
cargo install wasm-pack
npm install
wasm-pack build --target web --release
cd ..
rm -rf public/pkg
cp -r rust-backend/pkg/ public/pkg
npm run build
cp staticwebapp.config.json dist/staticwebapp.config.json