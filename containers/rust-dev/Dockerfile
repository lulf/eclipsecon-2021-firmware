FROM docker.io/library/rust:1-bullseye

RUN \
     curl -sSL https://github.com/rustwasm/wasm-pack/releases/download/v0.10.0/wasm-pack-v0.10.0-x86_64-unknown-linux-musl.tar.gz -o wasm-pack.tar.gz \
  && tar --strip-components=1 -xvzf wasm-pack.tar.gz --wildcards '*/wasm-pack' \
  && rm -Rf wasm-pack.tar.gz \
  && cp wasm-pack /usr/local/cargo/bin/ && rm wasm-pack

RUN \
     curl -sSL https://github.com/rustwasm/wasm-bindgen/releases/download/0.2.75/wasm-bindgen-0.2.75-x86_64-unknown-linux-musl.tar.gz -o wasm-bindgen.tar.gz \
  && tar --strip-components=1 -xvzf wasm-bindgen.tar.gz --wildcards '*/wasm-bindgen' \
  && rm -Rf wasm-bindgen.tar.gz \
  && cp wasm-bindgen /usr/local/cargo/bin/ && rm wasm-bindgen

RUN \
    curl -sSL https://github.com/WebAssembly/binaryen/releases/download/version_97/binaryen-version_97-x86_64-linux.tar.gz -o binaryen.tar.gz \
 && tar --strip-components=2 -xzvf binaryen.tar.gz '*/wasm-opt' \
 && rm -Rf binaryen.tar.gz \
 && cp wasm-opt /usr/local/cargo/bin/ && rm wasm-opt
