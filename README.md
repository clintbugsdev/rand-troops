# Getting Started

This guide will help you get your system set up for building this project. 
### Tip
For best result and running output from this project. Please use Windows Subsystem for Linux when using Windows OS or run it in any the Linux or MacOS machine.

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
This command installs a package and any packages that it depends on.
```bash
npm install
```

## Sample Usage
#### 
```javascript
// Filename index.js

// import random-troops.js module
const randomTroops = require("./random-troops");

// set the parameters
let player = "Player 1";
let armyTypes = ["Spearmen", "Swordsmen", "Archer"];
let armySize = 10;

// call the randomTroops fund and prints result
console.log(randomTroops(player, armyTypes, armySize));
```

## Test
#### 
```bash
# Run random-troops.test.js
npm jest
```
