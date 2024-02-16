#!/bin/bash
cd ./rust-backend/
wasm-pack build --target web
cd ..
rm -rf public/pkg
cp -r rust-backend/pkg/ public/pkg
npm run build