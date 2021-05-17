# Wdr

Wdr is a client-server process control system.

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

## Wdr Manager API

### Get configuration

Request:

```
GET /configs
```

Response:

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

### Set configuration

Request:

```json
PUT /configs
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

### Get node list

Request:

```
GET /nodes
```

Response:

```json
[
    "node-a-192.66.66.66",
    "node-b-192.77.77.77",
    "node-c-192.88.88.88"
]
```

### Get node info

Request:

```
GET /nodes/{node_name}/info
```

Response:

```json
{
    "processInfoList": [
        {
            "name": "hello",
            "version": "1",
            "state": "running"
        },
        {
            "name": "world",
            "version": "1",
            "state": "downloading"
        }
    ]
}
```

### Delete a node

Request:

```
DELETE /nodes/{node_name}
```
