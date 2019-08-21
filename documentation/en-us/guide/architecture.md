# Architecture

## 1. Vision and Mission

### 1.1 Core Vision
**Establish a unified, stable, and high-performance ORM + RPC integrated solution in the Rust ecosystem, providing a complete toolchain for building microservices.**

### 1.2 Mission Statement
1. **Integration**: Unify data model definition and RPC service definition, eliminating the pain points of duplicate writing and inconsistent updates.
2. **High Performance**: Zero-overhead abstraction, compile-time generation of optimal Rust code, no runtime reflection.
3. **Database First**: Each .rbq file corresponds to a logical database, bound to physical data sources through TOML configuration.
4. **Modular**: Reference between files via using, supporting cross-file reuse.
5. **Maintainability**: Clear architectural boundaries, reducing technical debt.

## 2. Design Philosophy

### 2.1 Core Principles

#### **Principle 1: Integration of Modeling, ORM, and RPC**
The RBQ engine is responsible for "expressing intent" (RBQ Language), "physical execution" (RBQ Core), and "remote communication" (xRPC implementation). This integration allows the compiler to deeply optimize logical queries and service definitions based on an understanding of physical characteristics and communication requirements.

#### **Principle 2: Layered Abstraction, Separation of Concerns**
- **Modeling & Translation Layer** → Responsible for declarative modeling, DSL parsing, and code generation.
- **Execution & Drivers Layer** → Responsible for physical connection management and protocol adaptation.
- **RPC & Communication Layer** → Responsible for remote service calls, middleware, and flow control.

#### **Principle 3: Respect Protocol Differences**
We do not attempt to completely shield the characteristics of underlying databases and network protocols, but expose these characteristics through a unified API, with the driver layer and transport layer handling specific protocol details.

## 3. Architecture Overview

### 3.1 Core Responsibility Matrix

| Component | Responsible For | Not Responsible For |
|-----------|-----------------|---------------------|
| **RBQ Language** | Declarative modeling, DSL parsing, Schema migration management, business constraint verification, RPC service definition | Physical connection management, driver low-level implementation, network transmission |
| **RBQ-Core** | Unified execution interface (Connection/Transaction), driver extension hooks, async runtime adaptation | Business logic, SQL generation details |
| **RBQ-Types** | Cross-driver basic data type mapping (Value), physical error classification (Error) | Business entity types |
| **RBQ-Pool** | Connection pool warm-up, health checks, automatic recovery | Specific SQL syntax translation |
| **xRPC Implementation** | Remote service calls, middleware chain, flow control, service discovery | Database operations, business logic |

## 4. Core Technical Components Design

### 4.1 Unified Execution API

#### **Design Goals**
- Provide the same execution method signatures for all databases.
- Maintain type safety and minimize runtime overhead.
- Support both synchronous and asynchronous execution modes.

#### **Core Interface Definition**
```rust
#[async_trait]
pub trait Database: Clone + Send + Sync + 'static {
    type Row: Row;
    async fn query_one(&self, sql: &str, params: &[&dyn ToSql]) -> Result<Option<Self::Row>, DbError>;
    async fn query_all(&self, sql: &str, params: &[&dyn ToSql]) -> Result<Vec<Self::Row>, DbError>;
    async fn execute(&self, sql: &str, params: &[&dyn ToSql]) -> Result<u64, DbError>;
    async fn transaction(&self) -> Result<Box<dyn Transaction>, DbError>;
}

pub trait Row {
    fn get<T: FromSql>(&self, idx: usize) -> Result<T, DbError>;
    fn get_by_name<T: FromSql>(&self, name: &str) -> Result<T, DbError>;
}
```

### 4.2 Driver Protocols & Security

#### **SQL Injection Prevention**
RBQ prohibits string concatenation at the driver level. All drivers must implement parameterized queries via native database protocols:
- **MySQL**: Uses `StmtPrepare` and `StmtExecute` binary protocols.
- **PostgreSQL**: Uses `Parse`, `Bind`, `Describe`, `Execute` extended query protocols.

This design ensures that parameters are completely separated from SQL logic at the protocol level, eliminating SQL injection risks at the source.

#### **Transport Security (TLS/SSL)**
All drivers include built-in `rustls` support, providing out-of-the-box encrypted transmission to ensure data confidentiality and integrity during network transit.

### 4.3 Connection Pooling

#### **Advanced Maintenance Mechanisms**
`rbq-pool` is more than just a connection container; it features automated maintenance capabilities:
- **Health Checks (Test on Borrow)**: Automatically executes `ping` or simple queries before acquiring a connection to ensure availability.
- **Background Maintenance**: Independent background tasks periodically clean up idle connections exceeding `max_idle_time`.
- **Dynamic Replenishment**: Automatically maintains a `min_idle` number of active connections to ensure responsiveness during traffic peaks.
- **Fair Competition**: A semaphore-based acquisition mechanism ensures fairness and stability under high concurrency.

### 4.4 xRPC Implementation Architecture

#### **Layered Architecture**
- **Transport Layer**: Manage underlying connections and data frame transmission, supporting TCP, Unix Domain Socket, shared memory.
- **Message Layer**: Serialization/deserialization, compression, metadata pass-through, based on Prost (Protobuf) but pluggable.
- **RPC Layer**: Method dispatching, middleware chain, flow control, supporting unary, client streaming, server streaming, bidirectional streaming.
- **Governance Layer**: Load balancing, service discovery, gray routing, built-in round-robin, consistent hashing.

#### **Protocol Design**
- **Frame Format**: `[length:4 bytes][message body]`, message body is Protobuf-encoded `xrpc.Request` / `xrpc.Response` encapsulation.
- **Multiplexing**: Each connection can have multiple concurrent streams, distinguished by stream ID.
- **Flow Control**: Window-based dynamic flow control to prevent receiver overload.

### 4.5 Observability

#### **Structured Logging & Tracing**
Full integration with the `tracing` framework covers the entire lifecycle from connection pool acquisition, query execution to connection closure. Supports distributed tracing for easy identification of performance bottlenecks.

#### **Metrics Monitoring**
`PoolMetrics` exposes core real-time metrics:
- Number of threads waiting for connections and average wait duration.
- Statistics for connection creation, closure, and timeouts.
- Success and failure counts for query execution.

### 4.6 Error Handling System
RBQ establishes a standardized error mapping mechanism. All native errors from underlying drivers (e.g., MySQL's `ER_ACCESS_DENIED_ERROR` or PostgreSQL's `28P01`) are mapped to the `RBQError` enum. This allows upper-level frameworks to implement unified retry or error reporting logic without worrying about differences in underlying database error codes.

## 5. Database Driver Implementation Details

RBQ's core philosophy is "Protocol Respect." Each driver is meticulously implemented according to the database's native protocol while maintaining a consistent strategy in security and compatibility.

### 5.1 MySQL Driver Details
*   **Auth Protocol Compatibility**:
    *   RBQ handles MySQL's `AuthSwitchRequest` (`0xFE` packet), allowing the server to dynamically request a change in the authentication plugin during the handshake.
    *   Currently, it focuses on supporting the `mysql_native_password` plugin, ensuring password security during transmission through a SHA1 double-hashing mechanism (Scramble mechanism).
*   **Packet Handling**:
    *   Implements a streamlined packet parser capable of efficiently handling OK, ERR, and ResultSet packets.
    *   Strictly follows the MySQL binary protocol for parameterized queries via `COM_STMT_PREPARE` and `COM_STMT_EXECUTE`.

### 5.2 PostgreSQL Driver Details
*   **V3 Protocol Support**:
    *   Fully supports the PostgreSQL interactive binary V3 protocol.
    *   Connection initialization uses `StartupMessage`, and connection availability is accurately managed based on the `ReadyForQuery` state machine.
*   **Advanced Error Mapping**:
    *   Parses PostgreSQL's `ErrorResponse` fields (such as SQLSTATE), providing richer contextual information than standard errors.

### 5.3 Unified Strategies
*   **Mandatory Parameter Binding**: No driver-level interfaces provide string concatenation; all parameter passing must use the `Value` array.
*   **Transparent Connection State**: Real-time tracking of connection status as `Idle`, `Busy`, or `InTransaction` via `ConnectionState`.
*   **Automated Resource Cleanup**: Leverages Rust's `Drop` mechanism and connection pool management to ensure database resources (like cursors or prepared statements) are correctly cleaned up when connections are returned.

#### **Unified Error Hierarchy**
```
RBQError (Top-level Error)
├── ConnectionError
├── QueryError
├── TransactionError
├── TypeConversionError
└── DatabaseError
    ├── PostgresError
    ├── MySQLError
    └── ...
```

## 6. Code Generation Architecture

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
   - Generate service traits and client/server code (based on xRPC implementation).
   - Generate stream type wrappers.

5. **Output**: Write generated files to `output_dir` and generate `mod.rs` for unified export.

### 6.1 Generated Code Structure

Taking `user.rbq` as an example, generated code structure:

```
src/user/
├── mod.rs                # Module entry, re-exporting public items
├── models.rs              # Model entities (User, ...)
├── db.rs                  # Database operations (UserOps, QueryBuilder, transactions)
├── rpc.rs                 # RPC messages and service definitions (UserService trait, client, server)
└── streams.rs             # Stream types (such as UserStream, ChatMessageStream)
```
