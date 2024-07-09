# Tauri + React + Vite + Tailwind CSS

This template is only for the strong minds.

Make sure you have Rust installed and rust toolchain upto date.
INSTALL TAURI CLI:
cargo install tauri-cli
Or npm install --save-dev @tauri-apps/cli

To Run:
npm run tauri dev
or 
cargo tauri dev

Further Documentation are under developer.

# Tauri + React + Vite + Tailwind CSS

This template is designed for developers with Strong-mind.

## Prerequisites

Ensure you have Rust installed and your Rust toolchain is up-to-date.

## Installation

### Install Tauri CLI

Using Cargo:
```sh
cargo install tauri-cli
```

Using Npm:
```sh
npm install --save-dev @tauri-apps/cli
```

## Running

### Install Dependencies

cd into repo.
Using Npm:
```sh
npm install
```

### Run Tauri

Using Npm:
```sh
npm tauri run dev
```
cd into src-tauri.
Using Cargo:
```sh
cargo tauri dev
```

## Building

### macOS Bundle
running 
```sh
rustup target add aarch64-apple-darwin
```
or 
```sh
rustup target add x86_64-apple-darwin
```
then you can build your app using:

* targets Apple silicon 
```sh
cargo tauri build --target aarch64-apple-darwin
```

* targets Intel-based
```sh
cargo tauri build --target x86_64-apple-darwin
```

* universal macOS binary that runs on both Apple silicon and Intel-based Macs.
```sh
cargo tauri build --target universal-apple-darwin
```

### Windows Installer
// todo
* Updating ...