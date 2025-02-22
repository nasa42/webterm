# Building Webterm

## Building the Relay

### Prerequisites

* Webterm Relay is built in Rust. You can install Rust and Cargo by following instructions
  from [The Cargo Book](https://doc.rust-lang.org/cargo/getting-started/installation.html).

### Build the Relay

```bash
git clone https://github.com/nasa42/webterm
cd relay
cargo build
```

### Run the Relay locally

```bash
# By default the relay runs on port 4200 and binds to localhost
# Append `-- --help` to see all available options
cargo run --bin webterm-relay
```

### Deploy the Relay

See doc: [Deploy the Relay](./deploy-relay.md)

## Building the Agent

### Prerequisites

* Webterm agent is built in Rust. You can install Rust and Cargo by following instructions
  from [The Cargo Book](https://doc.rust-lang.org/cargo/getting-started/installation.html).
* Building agent requires OpenSSL. On an Ubuntu/Debian system, install it with:

```bash
sudo apt-get install libssl-dev
```

### Build the Agent

```bash
git clone https://github.com/nasa42/webterm
cd agent
cargo build
```

### Run the Agent

```bash
# Run the agent and connect to the locally running relay
cargo run --bin webterm-agent -- --device-name "<device name>" --secret-key "<secret key>" --relays localhost:4200
```

## Building the Frontend

* Webterm frontend is a static site built using [Astro](https://github.com/withastro/astro). Ensure you've Node
  installed or install it
  with [these instructions](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm). Node version should
  match the version mentioned in [.node-version](../frontend/.node-version) file.

### Run the Frontend locally

1. Clone the repository and install dependencies
    ```bash
    git clone https://github.com/nasa42/webterm
    cd frontend
    npm install
    ```
2. Create `frontend/.env.development` with following:
    ```
    PUBLIC_DEFAULT_RELAYS=http://localhost:4200
    ```
3. Run the local Astro server
    ```
    # Ensure you're in the "frontend" directory
    npm run dev
    ```

### Deploy the Frontend

See doc: [Deploy the Frontend](./deploy-frontend.md)
