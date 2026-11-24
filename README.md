# Dandelion
Dandelion is a p2p chat app. Messages travel directly between devices, so there's no central server reading, storing, or owning your conversations. Like seeds on the wind, your messages go straight from one person to another.

## Features

## How It Works

Messages are directed through a server, and may be cached temporarily. However the server will never have the ability to read any messages, and will not continue to store them after they are delivered.

## Getting Started

### Prerequisites
### Installation

#### Client

```bash
git clone git@github.com:pewguin/dandelion.git # Clone the project
cd dandelion
nix develop # Enter the shell environment (not necessary if cargo is already installed, but recommended)
cargo build --bin client --release # Build the client binary
```
The binary will be at dandelion/target/release/client

#### Server

```bash
git clone git@github.com:pewguin/dandelion.git # Clone the project
cd dandelion
nix develop # Enter the shell environment (not necessary if cargo is already installed, but recommended)
cargo build --bin server --release # Build the server binary
```
The binary will be at dandelion/target/release/server
