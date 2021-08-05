# eclipsecon 2021 demo

## Builing microbit example

```
cargo build --target thumbv7em-none-eabihf --features microbit --release
```

Press the 'A' button to light the LED.

## Building the wasm example

```
wasm-pack build --target web -- --features wasm

# Start web server
python3 -m http.server
```

Then open your browser at http://localhost:8080
