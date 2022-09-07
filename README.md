# Rust Lang Web Framework Salvo

React app is accessed via /index.html not just /.

## Creating Commands

```bash
cargo new rust-salvo-ex1
cargo rumn --release
```

```bash
sudo apt install wrk
wrk -t8 -c256 -d30s http://127.0.0.1:3000/
```

## Code History

The content of this repository is based on the
[is salvo really the simplest rust web framework](https://youtu.be/tf9x97eTcpk)
video.
