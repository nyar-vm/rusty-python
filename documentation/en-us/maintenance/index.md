# Maintenance Guide

This section is for core contributors and advanced users of RBQ, delving into the engine's internal implementation, compiler principles, and underlying adaptation mechanisms.

## Core Sections

### Compiler and Internal Implementation
- **[Compiler Internal Implementation and Extension Mechanism](internals.md)**：Deeply understand the IR pipeline, type mapping, and driver adapter interface.
- **[Tokio Ecosystem Integration](tokio-integration.md)**：Learn how RBQ deeply adapts to the Tokio ecosystem.

### Database Drivers
- **[Database Driver Design and New Addition Guide](database/index.md)**：Master how to add new database support for RBQ.
- **[MySQL Driver](database/mysql.md)**：Implementation details and feature support of MySQL driver.
- **[PostgreSQL Driver](database/pgsql.md)**：Implementation details and feature support of PostgreSQL driver.
- **[SQLite Driver](database/sqlite.md)**：Implementation details and feature support of SQLite driver.
- **[Redis Driver](database/redis.md)**：Implementation details and feature support of Redis driver.

## Maintenance Goals

1. **Type Consistency**：Ensure that all driver backends have consistent support for RBQ scalar types.
2. **Performance Benchmarks**：Monitor the efficiency of HIR to LIR conversion, optimize physical execution plan generation.
3. **Dialect Coverage**：Continuously enhance support for new features of mainstream databases (PostgreSQL, MySQL, SQLite, Redis).
4. **xRPC Implementation**：Ensure high performance and reliability of RPC services.

## Architecture Overview

RBQ's architecture design follows clear layering principles：

1. **DSL Layer**：Use `.rbq` files to define data models and RPC services.
2. **Compiler Layer**：Convert DSL into high-performance Rust code, including ORM and RPC functionality.
3. **Runtime Layer**：Provide database connections, transaction management, and RPC service runtime.
4. **Driver Layer**：Adapt to different database systems, handle dialect differences.

## Contribution Guide

1. **Code Style**：Follow the official Rust code style guide.
2. **Test Coverage**：Ensure all new features have corresponding test cases.
3. **Documentation Updates**：Synchronously update related documentation to ensure consistency between documentation and code.
4. **Performance Considerations**：Focus on compile-time and runtime performance, avoid unnecessary overhead.