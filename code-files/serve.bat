cd .\rust-backend\
wasm-pack build --target web
cd ..
rmdir /S /Q public\pkg
xcopy /E /I rust-backend\pkg public\pkg\
npm run serve
