# Prerequisites

## 1. You can just [install Docker Desktop](https://www.docker.com/products/docker-desktop/).

Or, you could install [Rust](https://www.rust-lang.org/tools/install) and [WasmEdge](https://wasmedge.org/docs/develop/build-and-run/install) as follows.

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup target add wasm32-wasi

curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | sudo bash -s -- -p /usr/local
```

```
├── Cargo.lock
├── Cargo.toml
├── control
├── function
├── hello
├── move
├── README.md
├── server
├── string
├── struct
└── wasi
```

## 2. Install Docker Desktop and turn on the containerd image store feature in your Docker Desktop settings.

## 3. Setting up Docker Buildx with WASI/Wasm Support

This guide will walk you through the steps to set up Docker Buildx with support for building images targeting the WebAssembly System Interface (WASI) platform.

Enable Docker Experimental Features:

Open the Docker daemon configuration file. The location of the file varies depending on your operating system:

- Linux: /etc/docker/daemon.json
- macOS: ~/.docker/daemon.json
- Windows: %USERPROFILE%\.docker\daemon.json
- If the file doesn't exist, create it.

Add the following configuration to enable Experimental mode:
```
{
  "experimental": true
}
```
Create a Docker Buildx Builder Instance:

Open a terminal or command prompt.

Run the following command to create a new builder instance and set it as the active builder:

```
docker buildx create --use --name mybuilder --platform wasi/wasm
```
This command creates a new builder instance with the name "mybuilder" and enables it to build images targeting the WASI platform.

Verify the Builder Instance:

Run the following command to list the available builder instances and ensure that the "mybuilder" instance is listed:

```
docker buildx ls
```
This command should display the list of builder instances, including the "mybuilder" instance with the WASI platform.

You have now successfully set up Docker Buildx with support for building images targeting the WebAssembly System Interface (WASI) platform. You can use this builder instance for multi-platform builds, including building Docker images specifically for WASI/Wasm applications.

