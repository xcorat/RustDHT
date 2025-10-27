# RustDHT Server Deployment Guide

## Binary Installation

### Prerequisites
- None (binary is statically linked)

### Installation Steps
1. Download the appropriate binary for your platform:
   - Linux: `rustdht-server-linux-x86_64`
   - Windows: `rustdht-server-windows-x86_64.exe`
   - macOS: `rustdht-server-macos-x86_64`

2. Make the binary executable (Linux/macOS):
   ```bash
   chmod +x rustdht-server-*
   ```

3. Run the server:
   ```bash
   ./rustdht-server
   ```

## Docker Installation

1. Pull the Docker image:
   ```bash
   docker pull your-registry/rustdht-server:latest
   ```

2. Run the container:
   ```bash
   docker run -d \
     --name rustdht-server \
     -p 8765:8765 \
     your-registry/rustdht-server:latest
   ```

## Configuration

The server listens on port 8765 by default. You can modify this using environment variables:

```bash
# Binary
export RUSTDHT_PORT=9000
./rustdht-server

# Docker
docker run -d \
  --name rustdht-server \
  -p 9000:9000 \
  -e RUSTDHT_PORT=9000 \
  your-registry/rustdht-server:latest
```

## Health Check

The server provides a health check endpoint at `/health`:
```bash
curl http://localhost:8765/health
```

## Logging

Logs are written to stdout/stderr. When running with systemd, logs can be viewed with:
```bash
journalctl -u rustdht-server
```

In Docker:
```bash
docker logs rustdht-server
```

## Systemd Service (Linux)

Create a systemd service file at `/etc/systemd/system/rustdht-server.service`:

```ini
[Unit]
Description=RustDHT Signaling Server
After=network.target

[Service]
ExecStart=/usr/local/bin/rustdht-server
Restart=always
User=rustdht
Group=rustdht
Environment=RUSTDHT_PORT=8765

[Install]
WantedBy=multi-user.target
```

Enable and start the service:
```bash
sudo systemctl enable rustdht-server
sudo systemctl start rustdht-server
```

## Security Considerations

1. The server should be run behind a reverse proxy (like Nginx) in production
2. Configure SSL/TLS termination at the proxy level
3. Consider implementing rate limiting
4. Monitor server resources and implement appropriate limits

## Monitoring

The server exposes metrics that can be scraped by Prometheus at `/metrics`.

Common metrics:
- Connected peers count
- Message relay latency
- Connection errors
- Resource usage

## Troubleshooting

Common issues and solutions:

1. Port already in use:
   ```bash
   sudo lsof -i :8765
   ```

2. Connection refused:
   - Check firewall settings
   - Verify the server is running
   - Ensure correct port forwarding (if using Docker)

3. High latency:
   - Check network conditions
   - Monitor system resources
   - Consider scaling horizontally

## Support

For issues and feature requests, please open an issue on GitHub.
