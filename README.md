# Ping Latency App (Rust + Yew + WASM)

A lightweight client-side web app built with **Rust**, **Yew**, and **WebAssembly** that measures **HTTP latency** to `example.com` and displays the duration in milliseconds.

> ⚠️ Note: Browsers do **not** allow ICMP ping. This app measures **HTTP request latency**, not real network ping.

---

## 🚀 Live Demo

https://yashpatel-07.github.io/napp/

---

## 🛠 Tech Stack

- Rust
- Yew
- WebAssembly (WASM)
- Trunk
- GitHub Pages

---

## 📦 Prerequisites

- Rust (stable)
- wasm32 target

```bash
rustup target add wasm32-unknown-unknown
```
