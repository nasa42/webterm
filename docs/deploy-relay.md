# Deploy Relay

This guide explains how to deploy Webterm Relay on an Ubuntu server.

## Prerequisites

* Have a registered domain name (we'll use `relay.example.com` as an example)
* Configure DNS to point to your Ubuntu server

## Server Setup

### Caddy Installation

We'll use Caddy as the web server for its automatic HTTPS support.

1. Install Caddy
    ```bash
    apt-get install caddy
    ```

2. Configure Caddy by replacing `/etc/caddy/Caddyfile` with:
    ```
    relay.example.com {
      reverse_proxy localhost:4200
    }
    ```

### Systemd Configuration

1. Create a systemd unit file at `/etc/systemd/system/webterm-relay.service`:
    ```
    [Unit]
    Description=webterm-relay
    After=network.target
    
    [Service]
    Type=simple
    ExecStart=/usr/bin/webterm-relay
    Restart=always
    
    [Install]
    WantedBy=multi-user.target
    ```

2. Enable the service:
    ```bash
    systemctl enable webterm-relay.service
    ```

### Relay Installation

1. Download and install the Relay:
   ```bash
   curl -sSfL https://webterm.run/install-relay.sh | bash
   ```
   (you can also build from source, see [Build Instructions](./build-instructions.md) doc)

2. Start the Relay service:
    ```bash
    systemctl start webterm-relay.service
    ```

3. Apply the configuration by restarting Caddy:
    ```bash
    systemctl restart caddy
    ```

## Installation Verification

Visit `https://relay.example.com/up` in your browser. You should receive a JSON response with `status: ok`.
This endpoint can also be used for uptime monitoring.

## Agent Configuration

To connect a Webterm Agent to your relay, use the `--relays` option:

```bash
webterm-agent --relays relay.example.com
```

## Upgrading

To upgrade the Relay, repeat the "Relay Installation" steps.

That's it!
