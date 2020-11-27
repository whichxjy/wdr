# Wdr

Wdr is client-server process control system.

## Design

![wdr-design](docs/wdr-design.png)

## Wdr (Server Side)

```
RUST_LOG=info cargo run -p wdr
```

## Wdr Manager (Client Side)

```
RUST_LOG=info cargo run -p wdrm
```

## Configuration

```json
{
    "configs": [
        {
            "name": "hello",
            "version": "1",
            "resource": "https://example.com/path/to/hello",
            "cmd": "./hello"
        },
        {
            "name": "world",
            "version": "1",
            "resource": "https://example.com/path/to/world",
            "cmd": "./world"
        }
    ]
}
```
