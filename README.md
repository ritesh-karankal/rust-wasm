# rust-wasm

Resource:
https://wasmedge.org/docs/develop/build-and-run/docker_wasm/

The README for this project follows a template from: 
https://github.com/second-state/rust-examples/tree/main/hello

## Prerequisites
Please 

## Quick start with Docker

```
$ docker run --rm --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm riteshkarankal/rust-wasm:latest
Ahoy there!
You be running on an OS () with an ARCH of (wasm32)!
```

## Code

The [`src/main.rs`](src/main.rs) source code shows
It imports the consts module from the std::env module in the Rust standard library. This module provides constants related to the current environment.

The main function is the entry point of the program.

The first println! macro prints the message "Ahoy there!" to the console.

The second println! macro uses the constants provided by consts to print information about the operating system and architecture. It uses the consts::OS constant to retrieve the name of the current operating system and consts::ARCH constant to retrieve the architecture information.

Finally, the program will exit.

## Step by step guide

Compile the Rust source code project to a Wasm bytecode file.

```
$ cargo build --target wasm32-wasi --release
```

Run the Wasm bytecode file in WasmEdge CLI.

```
$ wasmedge target/wasm32-wasi/release/rust-wasm.wasm
Ahoy there!
You be running on an OS () with an ARCH of (wasm32)!
```
The runtime environment provides a sandboxed execution environment that isolates code from the underlying OS, ensuring security and portability. 

## Build and publish on Docker

The `Dockerfile` follows the above steps to build and package a lightweight OCI-compliant container image for the Wasm app.
Now, we need to publish the container image to Docker Hub.
You just need to specify that the WasmEdge application image is for the `wasi/wasm` platform.

```
$ $ docker buildx build --platform wasi/wasm -t riteshkarankal/rust-wasm .
... ...
$ docker push riteshkarankal/rust-wasm
```

