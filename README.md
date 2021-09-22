# EclipseCon 2021 demo

## Device firmware

NOTE: You will need a Micro:bit v2 for this demo.

### Builing microbit example

```
RUSTFLAGS='-C link-arg=--nmagic -C link-arg=-Tlink.x' cargo build --target thumbv7em-none-eabihf --features microbit --release
```

### Running microbit example

```
probe-run --chip nrf52833_xxAA target/thumbv7em-none-eabihf/release/eclipsecon-demo
```

Press the 'A' button to light the LED.

### Building the wasm example

```
wasm-pack build --target web -- --features wasm

# Start web server
python3 -m http.server
```

Then open your browser at http://localhost:8000

## Cloud setup

### Minikube

Start Minikube:

```
minikube start --cpus 4 --memory 16384 --disk-size 20gb --addons ingress
```

Once it is up, start the tunnel (and leave it running):

```
minikube tunnel
```

### Eclipse Che

Follow the installation instructions from: https://www.eclipse.org/che/docs/che-7/installation-guide/installing-che-on-minikube/

NOTE: Skip the `minikube start` step in this tutorial, as you did this in the step before, with more resources.

## Import project

Import a new project from this repository: https://github.com/lulf/eclipsecon-2021-firmware
