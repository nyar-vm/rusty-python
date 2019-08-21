# xRPC Concept

## What is xRPC

xRPC is a high-performance RPC framework concept, which is one of the core design concepts of RBQ. xRPC aims to provide a complete toolchain for building high-performance microservices, emphasizing simplicity, efficiency, and flexibility.

## Design Philosophy of xRPC

1. **High Performance**: Adopting lock-free design, zero-copy serialization, fully utilizing Rust's async ecosystem.
2. **Simple Protocol**: Custom simple frame protocol, avoiding the complexity of HTTP/2.
3. **Flexible Extension**: Modular design, supporting multiple transport layers and serialization methods.
4. **Integration with ORM**: As part of the RBQ DSL, implementing unified definition of models and RPC services.

## Layered Architecture of xRPC

| Layer | Responsibility | Key Components |
|-------|----------------|----------------|
| **Transport Layer** | Manage underlying connections and data frame transmission | `FrameTransport` trait (supports TCP, Unix Domain Socket, shared memory) |
| **Message Layer** | Serialization/deserialization, compression, metadata pass-through | Based on Prost (Protobuf) but pluggable, supports LZ4/Zstd compression |
| **RPC Layer** | Method dispatching, middleware chain, flow control | `Service` trait (AFIT/RPITIT implementation for zero-overhead abstraction), supports unary, client streaming, server streaming, bidirectional streaming |
| **Governance Layer** | Load balancing, service discovery, gray routing | Built-in round-robin, consistent hashing, integration with xRPC naming service |

## Protocol Design of xRPC

xRPC uses a custom simple frame protocol:

- **Frame Format**: `[length:4 bytes][message body]`, message body is Protobuf-encoded `xrpc.Request` / `xrpc.Response` encapsulation.
- **Multiplexing**: Each connection can have multiple concurrent streams, distinguished by stream ID.
- **Flow Control**: Window-based dynamic flow control to prevent receiver overload.

## Performance Advantages of xRPC

- **Lock-free Transport**: Core path avoids using `std::sync::Mutex`, adopts atomic operations and channel design.
- **Zero Copy**: Use `Bytes` to share data as much as possible, reducing memory copying.
- **Async Native**: Based on `tokio`, fully utilizing Rust's async ecosystem.
- **Zero-overhead Abstraction**: All generated code uses static dispatch, no virtual function calls.
- **Memory Efficiency**: Use `Bytes` to avoid data copying, serialization/deserialization directly operates on byte buffers.

## Relationship between xRPC and RBQ

xRPC is one of the core design concepts of RBQ, and RBQ implements the xRPC concept through the following methods:

1. **DSL Integration**: Unified definition of data models and RPC services in `.rbq` files.
2. **Code Generation**: Compiler `rbqc` generates high-performance RPC client and server code.
3. **Type Safety**: All RPC methods and message types are type-safe.
4. **Multiple Communication Modes**: Support unary, client streaming, server streaming, bidirectional streaming communication modes.
5. **Middleware Support**: Through annotation system, support authentication, logging, monitoring and other middleware.

## Example

```rbq
// Service definition
service UserService {
    // Unary RPC
    get_user(request: GetUserRequest) -> User;

    // Server stream: return stream T
    list_users(request: ListUsersRequest) -> stream User;

    // Client stream: parameter is stream T
    create_users(requests: stream CreateUserRequest) -> BatchResponse;

    // Bidirectional stream
    chat(messages: stream ChatMessage) -> stream ChatMessage;
}
```

The generated Rust code can be used like this:

```rust
// Client
let client = UserServiceClient::connect("127.0.0.1:8080").await.unwrap();
let user = client.get_user(GetUserRequest { id: 1 }).await.unwrap();

// Server implementation
struct UserServiceImpl;

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get_user(&self, req: GetUserRequest) -> Result<User, RpcError> {
        // Implementation logic
    }
    
    async fn list_users(&self, req: ListUsersRequest) -> Result<impl Stream<Item = User>, RpcError> {
        // Implementation logic
    }
    
    // Other method implementations...
}

// Start server
let server = serve_user_service(UserServiceImpl);
server.listen("127.0.0.1:8080").await.unwrap();
```