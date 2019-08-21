# RPC Concept

## What is RPC

RPC (Remote Procedure Call) is a communication protocol that allows a program to call a subroutine in another address space (usually another computer on the network) without the programmer explicitly encoding the details of this remote call.

## Advantages of RPC

1. **Simplified Distributed Programming**: Developers can call remote services as if they were local functions, without worrying about the details of network communication.
2. **Improved Code Reuse**: Services can be shared by multiple clients, increasing code reuse.
3. **Service Decoupling**: Services communicate through clear interfaces, reducing coupling.
4. **Easy Scalability**: Each service can be independently scaled, improving system scalability.

## RPC Implementation in RBQ

RBQ has a built-in RPC system based on the xRPC concept, with the following features:

1. **Unified Definition**: Define data models and RPC services in the same `.rbq` file, eliminating duplicate definitions.
2. **Compile-time Code Generation**: Generate high-performance RPC client and server code at compile time.
3. **Type Safety**: All RPC methods and message types are type-safe, avoiding runtime errors.
4. **Multiple Communication Modes**: Support unary, client streaming, server streaming, bidirectional streaming communication modes.
5. **Middleware Support**: Through annotation system, support authentication, logging, monitoring and other middleware.
6. **High Performance**: Adopting lock-free design, zero-copy serialization, fully utilizing Rust's async ecosystem.

## Example

```rbq
// Message definition
message GetUserRequest {
    id: i64;
}

message UserList {
    users: User[];
    total: i64;
}

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

    // Can attach annotations
    @timeout(5s)
    @middleware("auth")
    delete_user(request: DeleteUserRequest) -> DeleteResponse;
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