# Redis Driver Implementation Details

The RBQ Redis driver supports RESP (Redis Serialization Protocol), providing efficient access to key-value stores and complex data structures.

## 1. RESP Protocol Implementation

RBQ implements a RESP protocol parser for Redis:
- **Simple Strings**: Parses replies starting with `+`.
- **Errors**: Parses error replies starting with `-` and converts them to `RBQError`.
- **Integers**: Parses integers starting with `:`.
- **Bulk Strings**: Handles binary-safe data, parsing replies starting with `$`.
- **Arrays**: Parses multi-element replies starting with `*`.

## 2. Command Execution Flow

1. **Formatting**: Encodes commands (e.g., `SET`, `GET`, `HGETALL`) and their arguments into RESP array format.
2. **Asynchronous Send**: Sends the encoded byte stream asynchronously via `TcpStream`.
3. **Response Reading**: Reads responses line-by-line via the parser, supporting pipelined batch operations.

## 3. RBQ Unified Adaptation

Although Redis is a NoSQL database, RBQ provides a unified interface adaptation for it:
- **Query Simulation**: Adapts common Redis read operations to the `query` interface.
- **Execute Simulation**: Adapts write operations to the `execute` interface.
- **Type Conversion**: Converts RESP types into RBQ's `Value` types.

## 4. Connection Management

- **Auto-Reconnect**: The driver includes basic retry logic.
- **Pub/Sub**: Supports Redis's Publish/Subscribe mechanism through `DriverExtension`.
