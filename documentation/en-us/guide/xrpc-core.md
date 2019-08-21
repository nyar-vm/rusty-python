# xRPC Core Design

xRPC is a self-developed, Rust-based high-performance RPC framework concept that deeply integrates with RBQ DSL, providing a complete toolchain for building high-performance microservices.

## 1. Design Philosophy

Core design principles of xRPC:

- **High Performance**: Adopting lock-free design, zero-copy serialization, fully utilizing Rust's async ecosystem.
- **Simple Protocol**: Custom simple frame protocol, avoiding the complexity of HTTP/2.
- **Flexible Extension**: Modular design, supporting multiple transport layers and serialization methods.
- **Deep Integration with RBQ**: As part of the RBQ DSL, implementing unified definition of models and RPC services.

## 2. Why Choose xRPC Over gRPC

Based on pure Rust, we can do better than gRPC—not only better, but for platforms pursuing extreme performance and "self-developed control", we should do better than gRPC.

This is not empty talk. Many teams have verified this path in practice:

- **ByteDance's Volo Framework**: Designed based on Rust's latest AFIT/RPITIT features, achieving 35W QPS under 4C limit, optimized version can reach 44W, and the framework itself's overhead is "basically negligible" in flame graphs.
- **Momento's protosocket**: This company was originally a heavy gRPC user, but found that lock competition inside `h2` caused task starvation. They developed the minimalist `protosocket`, **vertical scaling capability increased by 2.75 times**, and ultimately achieved **more than 10 times performance improvement** compared to gRPC (calculated by EC2 cost per dollar).
- **xRPC-rs's layered design**: Demonstrates seamless switching capability from shared memory to TCP, supports multiple serialization protocols, and has full link control.

### 🔍 What's the Problem with gRPC?

gRPC is an excellent general-purpose RPC framework, but it has the cost of "generality":

1. **HTTP/2 Complexity**: Lock competition (`std::sync::Mutex`) inside the `h2` library can cause threads to be suspended by `futex` under high concurrency, and one suspension is hundreds of microseconds—which is a huge overhead for systems pursuing microsecond-level responses.
2. **Overhead of Multiple Layers of Abstraction**: TLS + HTTP/2 + Protobuf + gRPC layer, each layer has abstraction costs. Although Rust has zero-cost abstraction, "zero cost" does not mean "no cost", and the combination of multiple layers is still considerable.
3. **Unable to Optimize for Scenarios**: Your business may be small packet high-frequency requests, or large packet streaming transmission, but gRPC can only provide a "general optimal solution", not a "scenario optimal solution" for you.
4. **Long Dependency Chain**: gRPC-rs depends on `tonic`, `tonic` depends on `h2` and `hyper`, once there is a performance bottleneck, the difficulty of troubleshooting and modification is extremely high.

### 💡 What Can Self-developed Rust RPC Bring?

#### 1. Extreme Performance Control

**Case: Momento's 10x Improvement**
After discovering gRPC's bottleneck, the Momento team did not patch the existing framework, but rewrote the minimalist `protosocket` in Rust:
- Removed the HTTP/2 wrapper layer, directly defined message frames on TCP
- Replaced `std::sync::Mutex` with self-developed `k_lock::Mutex` (more aggressive spin strategy)
- Result: Vertical scaling capability increased by 2.75 times, ultimately achieving **10x performance improvement**

**This means**: With the same 100 servers, your platform can carry service capabilities equivalent to others' 1000 servers—this is the technical barrier.

#### 2. Eliminate Unnecessary Abstractions

Rust's latest features allow us to write "true zero-cost abstractions":
- **AFIT (Async Fn in Trait) and RPITIT (Return Position Impl Trait in Trait)**: Volo uses these features to avoid `Box` dynamic dispatch and directly determine all call paths at compile time.
- **Static Dispatch**: All middleware and interceptors are expanded at compile time, no virtual function calls, no runtime type identification.

#### 3. Unify IPC and RPC

In the evolution of the platform (1 machine → 100 machines), the communication mode will go through:
- **Stage 1**: Inter-process communication within a single machine (shared memory, Unix Domain Socket)
- **Stage 2**: Multi-machine TCP communication in the same computer room
- **Stage 3**: Cross-region global communication

gRPC cannot well cover the "intra-machine" scenario—using TCP for loopback has too much overhead. While self-developed frameworks can do:

```rust
// Same API, different underlying transport
let transport = if cfg!(feature = "shared-memory") {
    SharedMemoryFrameTransport::new("/dev/shm/mos-ipc")?
} else {
    TcpFrameTransport::connect("10.0.0.1:9000")?
};
```

This is exactly the design idea of **xRPC**: from `ChannelFrameTransport` (in-process testing) to `SharedMemoryFrameTransport` (production IPC) to `TcpFrameTransport` (network deployment), the same interface evolves smoothly.

#### 4. Deep Integration with Ecosystem

Self-developed RPC framework can seamlessly integrate with your platform:
- **Integration with monitoring system**: Automatically inject tracing context at the framework layer to achieve full-link tracking
- **Integration with gray-scale system**: Implement gray-scale routing strategies on the client side (such as selecting service versions based on user ID hash)
- **Integration with file system**: Support dedicated optimization paths for large file streaming transmission

These deep integrations are difficult to implement elegantly under gRPC's general model.

## 3. Layered Architecture

xRPC adopts a clear layered architecture:

| Layer | Responsibility | Key Components |
|-------|----------------|----------------|
| **Transport Layer** | Manage underlying connections and data frame transmission | `FrameTransport` trait (supports TCP, Unix Domain Socket, shared memory) |
| **Message Layer** | Serialization/deserialization, compression, metadata pass-through | Based on Prost (Protobuf) but pluggable, supports LZ4/Zstd compression |
| **RPC Layer** | Method dispatching, middleware chain, flow control | `Service` trait (AFIT/RPITIT implementation for zero-overhead abstraction), supports unary, client streaming, server streaming, bidirectional streaming |
| **Governance Layer** | Load balancing, service discovery, gray routing | Built-in round-robin, consistent hashing, integration with xRPC naming service |

## 4. Protocol Design

xRPC uses a custom simple frame protocol:

- **Frame Format**: `[length:4 bytes][message body]`, message body is Protobuf-encoded `xrpc.Request` / `xrpc.Response` encapsulation.
- **Multiplexing**: Each connection can have multiple concurrent streams, distinguished by stream ID.
- **Flow Control**: Window-based dynamic flow control to prevent receiver overload.

## 5. Performance Advantages

- **Lock-free Transport**: Core path avoids using `std::sync::Mutex`, adopts atomic operations and channel design.
- **Zero Copy**: Use `Bytes` to share data as much as possible, reducing memory copying.
- **Async Native**: Based on `tokio`, fully utilizing Rust's async ecosystem.
- **Zero-overhead Abstraction**: All generated code uses static dispatch, no virtual function calls.
- **Memory Efficiency**: Use `Bytes` to avoid data copying, serialization/deserialization directly operates on byte buffers.

## 6. Integration with RBQ

xRPC deeply integrates with RBQ DSL:

- **Unified Definition**: Define data models and RPC services in the same `.rbq` file.
- **Code Generation**: Compiler `rbqc` generates complete RPC client and server code.
- **Type Safety**: All RPC methods and message types are type-safe.
- **Streaming Support**: Natively supports unary, client streaming, server streaming, bidirectional streaming communication modes.

## 7. Service Definition Example

```rbq
service UserService {
    // Unary RPC
    get_user(request: GetUserRequest) -> User;

    // Server stream: return stream T
    list_users(request: ListUsersRequest) -> stream User;

    // Client stream: parameter is stream T
    create_users(requests: stream CreateUserRequest) -> BatchResponse;

    // Bidirectional stream
    chat(messages: stream ChatMessage) -> stream ChatMessage;

    // Can attach annotations
    @timeout(5s)
    @middleware("auth")
    delete_user(request: DeleteUserRequest) -> DeleteResponse;
}
```

## 8. Generated RPC Code

The compiler will generate corresponding client and server code for each service:

```rust
// Service trait
#[async_trait]
pub trait UserService: Send + Sync + 'static {
    async fn get_user(&self, req: GetUserRequest) -> Result<User, RpcError>;
    async fn list_users(&self, req: ListUsersRequest) -> Result<impl Stream<Item = User>, RpcError>;
    // ...
}

// Client implementation
pub struct UserServiceClient {
    inner: xrpc::client::Client,
}

impl UserServiceClient {
    pub async fn get_user(&self, req: GetUserRequest) -> Result<User, RpcError> {
        self.inner.unary("/user.UserService/get_user", req).await
    }
    // ...
}

// Server skeleton
pub fn serve_user_service<S: UserService>(service: S) -> xrpc::Server {
    // Register method handlers
}
```

## 9. Stream Types

For streaming RPC, generate corresponding stream wrappers:

```rust
type UserStream = xrpc::stream::ReceiverStream<User>;
type ChatMessageStream = xrpc::stream::ReceiverStream<ChatMessage>;
```

## 10. Middleware Support

Through the annotation system, xRPC supports middleware:

- **Authentication**: Verify request identity information.
- **Logging**: Record requests and responses.
- **Monitoring**: Collect performance metrics.
- **Rate Limiting**: Control request rate.

## 11. Summary

As a philosophical concept and technical idea, xRPC provides RBQ with high-performance, flexible RPC capabilities. It deeply integrates with RBQ DSL, realizing unified definition of models and RPC services, eliminating the pain points of model duplication and inconsistent updates in traditional development.

Through compile-time code generation, xRPC ensures zero-overhead abstraction and type safety, while providing rich communication mode support. This design allows developers to focus on business logic while enjoying the extreme performance brought by Rust.