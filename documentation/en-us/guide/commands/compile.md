# Code Compilation (Compile)

The `rbqc` command converts `.rbq` model definitions into Rust code, including data models, database operations, and RPC services.

## Common Commands

### Basic Compilation
```bash
rbqc --config rbq.toml --output src
```

### Specify Configuration File
```bash
rbqc --config path/to/config.toml --output src
```

## Parameter Description

- `--config`, `-c`: Configuration file path, containing database connection information and compilation options.
- `--output`, `-o`: Output directory. If not specified, it will be generated according to configuration or default rules.

## Configuration File Options

Compilation-related options in the `rbq.toml` configuration file:

### [project] Section
- `source_dir`: RBQ source file directory, containing all `.rbq` files, default is "models"

### [databases] Section
- `url`: Database connection string, format depends on database type
- `schema`: Optional database schema name, such as "public" for PostgreSQL

### [generate] Section
- `target`: Target language, supporting "rust" or "typescript"
- `output`: Output directory for generated code

### [generate.rust] Sub-section
- `serde`: Whether to add serde serialization/deserialization support for generated types, default is false
- `runtime`: Whether to generate runtime support code, default is false

## Compilation Process

The compiler `rbqc` performs the following steps:

1. **Parsing**: Read all `.rbq` files and build AST.
2. **Linking**: Parse cross-file references based on `using` statements, build global symbol table.
3. **Verification**: Check types, constraints, circular dependencies, etc.
4. **Code Generation**:
   - Generate Rust modules for each file.
   - Generate model structs (`struct`) and serialization implementations (using `serde`).
   - Generate database operation layer (if database is bound):
     - CRUD methods.
     - Query builder.
     - Transaction support.
   - Generate RPC message structs.
   - Generate service traits and client/server code (based on xRPC).
   - Generate stream type wrappers.

5. **Output**: Write generated files to `output_dir` and generate `mod.rs` for unified export.

## Compilation Products

Taking `user.rbq` as an example, generated code structure:

```
src/user/
├── mod.rs                # Module entry, re-exporting public items
├── models.rs              # Model entities (User, ...)
├── db.rs                  # Database operations (UserOps, QueryBuilder, transactions)
├── rpc.rs                 # RPC messages and service definitions (UserService trait, client, server)
└── streams.rs             # Stream types (such as UserStream, ChatMessageStream)
```

### Generated Model Entities (`models.rs`)

```rust
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: i64,
    username: String,
    email: String,
    created_at: DateTime<Utc>,
    // Relationship fields (lazy loading)
    posts: Option<Vec<Post>>,  // Need to import Post
}
```

### Generated Database Operations (`db.rs`)

```rust
use crate::db::Database;   // Self-developed Database trait
use crate::models::User;

impl User {
    // Basic CRUD
    async fn find_by_id(db: &impl Database, id: i64) -> Result<Option<Self>, DbError> {
        // Generated parameterized query
    }

    async fn insert(&self, db: &impl Database) -> Result<Self, DbError> {
        // ...
    }

    // Query builder
    fn query() -> UserQueryBuilder {
        UserQueryBuilder::new()
    }
}

// Query builder
struct UserQueryBuilder {
    limit: Option<i64>,
    offset: Option<i64>,
    condition: Option<String>,  // Actually more complex, using expressions
}

impl UserQueryBuilder {
    fn filter<F>(mut self, f: F) -> Self where F: FnOnce(&UserExpr) -> Condition { ... }
    fn order_by<F>(mut self, f: F) -> Self where F: FnOnce(&UserExpr) -> OrderBy { ... }
    async fn fetch(self, db: &impl Database) -> Result<Vec<User>, DbError> { ... }
}
```

### Generated RPC Code (`rpc.rs`)

```rust
// Generated request/response messages
#[derive(Debug, Clone, Serialize, Deserialize)]
struct GetUserRequest {
    id: i64,
}

// Service trait
#[async_trait]
trait UserService: Send + Sync + 'static {
    async fn get_user(&self, req: GetUserRequest) -> Result<User, RpcError>;
    async fn list_users(&self, req: ListUsersRequest) -> Result<impl Stream<Item = User>, RpcError>;
    // ...
}

// Client
struct UserServiceClient {
    inner: xrpc::client::Client,
}

impl UserServiceClient {
    async fn get_user(&self, req: GetUserRequest) -> Result<User, RpcError> {
        self.inner.unary("/user.UserService/get_user", req).await
    }
    // ...
}

// Server skeleton
fn serve_user_service<S: UserService>(service: S) -> xrpc::Server {
    // Register method handlers
}
```

### Generated Stream Types (`streams.rs`)

```rust
type UserStream = xrpc::stream::ReceiverStream<User>;
type ChatMessageStream = xrpc::stream::ReceiverStream<ChatMessage>;
```