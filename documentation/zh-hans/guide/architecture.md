# 架构

## 1. 愿景与使命

### 1.1 核心愿景
**建立 Rust 生态中统一、稳定、高性能的 ORM + RPC 一体化解决方案，为构建微服务提供完整的工具链。**

### 1.2 使命声明
1. **一体化**：统一数据模型定义和 RPC 服务定义，消除重复编写和更新不一致的痛点。
2. **高性能**：零开销抽象，编译期生成最优 Rust 代码，无运行时反射。
3. **数据库优先**：每个 .rbq 文件对应一个逻辑数据库，通过 TOML 配置绑定物理数据源。
4. **模块化**：通过 using 实现文件间的引用，支持跨文件复用。
5. **可维护性**：清晰的架构边界，降低技术债务。

## 2. 设计哲学

### 2.1 核心原则

#### **原则一：建模、ORM 与 RPC 一体化**
RBQ 引擎同时负责“表达意图”（RBQ Language）、“物理执行”（RBQ Core）和“远程通信”（xRPC 实现）。这种集成允许编译器在了解物理特性和通信需求的基础上，对逻辑查询和服务定义进行深度优化。

#### **原则二：分层抽象，关注点分离**
- **建模与编译层** (Modeling & Translation) → 负责声明式建模、DSL 解析与代码生成。
- **执行与驱动层** (Execution & Drivers) → 负责物理连接管理与协议适配。
- **RPC 层** (RPC & Communication) → 负责远程服务调用、中间件和流控。

#### **原则三：尊重协议差异**
我们不试图完全屏蔽底层数据库和网络协议的特性，而是通过统一的 API 暴露这些特性，驱动层和传输层负责处理具体的协议细节。

## 3. 架构总览

### 3.1 核心职责矩阵

| 组件 | 负责 | 不负责 |
|------|------|--------|
| **RBQ Language** | 声明式建模、DSL 解析、Schema 迁移管理、业务约束验证、RPC 服务定义 | 物理连接管理、驱动底层实现、网络传输 |
| **RBQ-Core** | 统一执行接口 (Connection/Transaction)、驱动扩展钩子、异步运行时适配 | 业务逻辑、SQL 生成细节 |
| **RBQ-Types** | 跨驱动的基础数据类型映射 (Value)、物理错误分类 (Error) | 业务实体类型 |
| **RBQ-Pool** | 连接池预热、健康检查、自动恢复 | 具体的 SQL 语法翻译 |
| **xRPC 实现** | 远程服务调用、中间件链、流控、服务发现 | 数据库操作、业务逻辑 |

## 4. 核心技术组件设计

### 4.1 统一执行接口 (Unified Execution API)

#### **设计目标**
- 为所有数据库提供相同的执行方法签名
- 保持类型安全，最小化运行时开销
- 支持同步和异步执行模式

#### **核心接口定义**
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

### 4.2 驱动协议与安全 (Driver Protocols & Security)

#### **防 SQL 注入设计**
RBQ 拒绝在驱动层进行字符串拼接。所有驱动必须通过数据库原生协议实现参数化查询：
- **MySQL**: 使用 `StmtPrepare` 和 `StmtExecute` 二进制协议。
- **PostgreSQL**: 使用 `Parse`, `Bind`, `Describe`, `Execute` 扩展查询协议。

这种设计确保了参数与 SQL 逻辑在协议层完全分离，从根源上杜绝了 SQL 注入风险。

#### **传输安全 (TLS/SSL)**
所有驱动内置 `rustls` 支持，提供开箱即用的加密传输能力，确保数据在网络传输过程中的机密性与完整性。

### 4.3 连接池管理 (Connection Pooling)

#### **高级维护机制**
`rbq-pool` 不仅仅是一个连接容器，它具备以下自动化维护能力：
- **健康检查 (Test on Borrow)**：在获取连接前自动执行 `ping` 或简单查询，确保连接可用。
- **后台自动维护**：独立的后台任务定期清理超过 `max_idle_time` 的空闲连接。
- **动态补充**：自动维持 `min_idle` 数量的活跃连接，确保业务高峰时的响应速度。
- **公平竞争**：基于信号量（Semaphore）的连接获取机制，确保高并发下的公平性与稳定性。

### 4.4 xRPC 实现架构

#### **分层架构**
- **传输层**：管理底层连接和数据帧传输，支持 TCP、Unix Domain Socket、共享内存。
- **消息层**：序列化/反序列化、压缩、元信息透传，基于 Prost（Protobuf）但可插拔。
- **RPC 层**：方法派发、中间件链、流控，支持一元、客户端流、服务端流、双向流。
- **治理层**：负载均衡、服务发现、灰度路由，内置轮询、一致性哈希。

#### **协议设计**
- **帧格式**：`[长度:4字节][消息体]`，消息体为 Protobuf 编码的 `xrpc.Request` / `xrpc.Response` 封装。
- **多路复用**：每个连接可并发多个 stream，通过 stream ID 区分。
- **流控**：基于窗口的动态流控，防止接收端过载。

### 4.5 可观测性 (Observability)

#### **结构化日志与追踪**
全量集成 `tracing` 框架，涵盖从连接池获取、查询执行到连接关闭的全生命周期埋点。支持分布式链路追踪，方便排查性能瓶颈。

#### **指标监控 (Metrics)**
`PoolMetrics` 实时暴露以下核心指标：
- 等待获取连接的线程数及平均等待时长。
- 连接创建、关闭及超时的次数统计。
- 查询执行的成功与失败计数。

### 4.6 错误处理系统 (Error Handling)
RBQ 建立了一套标准化的错误映射机制。所有驱动底层的原生错误（如 MySQL 的 `ER_ACCESS_DENIED_ERROR` 或 PostgreSQL 的 `28P01`）都会被映射为 `RBQError` 枚举。这使得上层框架无需关心底层数据库的错误码差异，即可实现统一的重试或报错逻辑。

## 5. 数据库驱动实现细节 (Database Driver Implementation Details)

RBQ 的核心理念是“协议尊重”。每个驱动都根据数据库的原生协议进行了精细化实现，同时在处理安全和兼容性方面保持了一致的策略。

### 5.1 MySQL 驱动处理细节
*   **认证协议兼容性**：
    *   RBQ 能够处理 MySQL 的 `AuthSwitchRequest`（`0xFE` 报文），允许服务器在握手阶段动态要求切换认证插件。
    *   目前重点支持 `mysql_native_password` 插件，通过 SHA1 二次哈希（Scramble 机制）确保密码在传输过程中的安全性。
*   **报文处理**：
    *   实现了精简的报文解析器，能够高效处理 OK、ERR 和 ResultSet 报文。
    *   严格遵循 MySQL 二进制协议进行参数化查询，通过 `COM_STMT_PREPARE` 和 `COM_STMT_EXECUTE` 实现。

### 5.2 PostgreSQL 驱动处理细节
*   **V3 协议支持**：
    *   全面支持 PostgreSQL 交互式二进制 V3 协议。
    *   连接初始化采用 `StartupMessage`，并能够根据 `ReadyForQuery` 状态机准确管理连接可用性。
*   **高级错误映射**：
    *   解析 PostgreSQL 的 `ErrorResponse` 字段（如 SQLSTATE），提供比标准错误更丰富的上下文信息。

### 5.3 统一处理策略
*   **参数绑定强制化**：所有驱动层均不提供字符串拼接接口，强制要求使用 `Value` 数组进行参数传递。
*   **连接状态透明化**：通过 `ConnectionState` 实时追踪连接是处于 `Idle`、`Busy` 还是 `InTransaction` 状态。
*   **资源回收自动化**：利用 Rust 的 `Drop` 机制和连接池管理，确保数据库资源（如游标、临时语句）在连接归还时得到正确清理。

#### **统一错误层次**
```
RBQError (顶级错误)
├── ConnectionError (连接错误)
├── QueryError (查询错误)
├── TransactionError (事务错误)
├── TypeConversionError (类型转换错误)
└── DatabaseError (数据库特定错误)
    ├── PostgresError (PostgreSQL错误)
    ├── MySQLError (MySQL错误)
    └── ...
```

## 6. 代码生成架构

编译器 `rbqc` 执行以下步骤：

1. **解析**：读取所有 `.rbq` 文件，构建 AST。
2. **链接**：根据 `using` 语句解析跨文件引用，构建全局符号表。
3. **验证**：检查类型、约束、循环依赖等。
4. **代码生成**：
   - 为每个文件生成 Rust 模块。
   - 生成模型结构体（`struct`）及序列化实现（使用 `serde`）。
   - 生成数据库操作层（如果绑定数据库）：
     - CRUD 方法。
     - 查询构建器。
     - 事务支持。
   - 生成 RPC 消息结构体。
   - 生成服务 trait 和客户端/服务器代码（基于 xRPC 实现）。
   - 生成流类型的包装。

5. **输出**：将生成的文件写入 `output_dir`，并生成 `mod.rs` 统一导出。

### 6.1 生成代码结构

以 `user.rbq` 为例，生成代码结构：

```
src/user/
├── mod.rs                # 模块入口，重新导出公共项
├── models.rs              # 模型实体（User, ...）
├── db.rs                  # 数据库操作（UserOps, QueryBuilder, 事务）
├── rpc.rs                 # RPC 消息和服务定义（UserService trait, client, server）
└── streams.rs             # 流类型（如 UserStream, ChatMessageStream）
```
