# SQLite Driver Implementation Details

The RBQ SQLite driver is a lightweight wrapper for the embedded database, providing efficient interaction with the SQLite C API via FFI.

## 1. Embedded Architecture

Unlike client/server models, the SQLite driver operates directly on files or memory:
- **Connection Management**: Each `SqliteConnection` holds a handle to an SQLite database instance.
- **Thread Safety**: Follows SQLite's `Serialized` threading model to ensure concurrency safety in Rust's asynchronous environment.

## 2. Prepared Statements

RBQ fully leverages SQLite's pre-compilation capabilities:
- **Caching Mechanism**: The driver layer caches pre-compiled common SQL statements to reduce parsing overhead.
- **Parameter Binding**: Uses the `sqlite3_bind_*` family of functions to ensure data is handled correctly at the bottom level, eliminating SQL injection.

## 3. Data Conversion

While SQLite is dynamically typed, RBQ provides strong type guarantees for it:
- **Storage Class Resolution**: Dynamically converts SQLite's five storage classes (NULL, INTEGER, REAL, TEXT, BLOB) into RBQ's `Value` types.
- **Column Association**: Retrieves result set column info at runtime via the `sqlite3_column_*` APIs and performs type validation.

## 4. Special Handling

- **In-Memory Databases**: Supports the `:memory:` connection string, suitable for testing and high-performance caching scenarios.
- **Shared Cache**: Supports SQLite's shared cache mode to optimize resource usage across multiple connections.
