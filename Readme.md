## Web builds

Make sure you have trunk installed (`cargo install --locked trunk`) and the web assembly target (`rustup target add wasm32-unknown-unknown`).

Run `trunk serve` for a development server that automatically re-compiles on changes.
Run `trunk build --release` for a web release (just zip up the `dist` directory).
