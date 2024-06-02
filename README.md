# wasm_ip_tools
A collection of tools to play with IP using rust-wasm. For my personal use, feel free to reuse it as much as you want.

```bash
# Build the wasm/js content
wasm-pack build --target web # You'll get a pkg folder.

# Launch a python server
python3 -m http.server 8002

# Now open your web browser on this url : http://0.0.0.0:8002/
```