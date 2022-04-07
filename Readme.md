## Web builds

Make sure you have trunk installed (`cargo install --locked trunk`) and the web assembly target (`rustup target add wasm32-unknown-unknown`).

Run `trunk serve` for a development server that automatically re-compiles on changes. Alternatively, try `trunk serve --release` if the former doesn't work.
Run `trunk build --release` for a web release (just zip up the `dist` directory).

## Itch.io

Before creating the zip above, the created `index.html` needs a few adjustments to work with itch.io:
* In the `<link rel ...>` lines, delete the `/` at the beginning of the path.
* In the last line of the script, add `./` to the beginning (i.e. `import init from './ ... .js'`).
* Delete the `/` from the following `init( ... )`.
* Now zip the dist folder and upload to itch.io :-)!
