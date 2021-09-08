# Getting Started

This guide will help you get your system set up for building this project. 

## Prerequisites

#### Node.js

This project requires Node.js and supports the latest Node version and all LTS releases. If you don't already have [Node.js](https://nodejs.org/) installed, or don't have a supported version, go to the for installation instructions.

#### Rust Programming Language

The project requires Rust for development. If you don't already have Rust installed, or don't have a supported version, go to the [Rust](https://www.rust-lang.org/install.html) web site for installation instructions.

Rust may have additional dependencies depending on your target platform. For example, Visual Studio on Windows.

#### Redis (Optional)

The project optionally requires Redis for temporary storage of generated random distributed troops. If you don't already have Redis installed, or don't have a supported version, go to the [Redis](https://redis.io/topics/quickstart) web site for installation instructions.


## Setup
#### 1. Run Redis-Server 
Open a new terminal window and let redis server run on your local machine while working on this project. 
```bash
redis-server
```
#### 2. Cargo
Use the package manager [cargo](https://doc.rust-lang.org/cargo/) to download the Rust package's dependencies and makes distributable package.
```bash
# Clean the installed rust packages if exists
cargo clean
```
```bash
# Build the rust library
cargo build --release
```
#### 3. NPM
This command installs a package and any packages that it depends on
```bash
npm install
```
