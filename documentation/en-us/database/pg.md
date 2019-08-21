# PostgreSQL Driver Implementation Details

The RBQ PostgreSQL driver is implemented based on the native binary V3 protocol, providing full asynchronous support and type safety.

## 1. Protocol Model

RBQ follows PostgreSQL's message-driven model:
- **Startup Message**: Sends a `StartupMessage` containing the username and database name after the connection is established.
- **Ready State (ReadyForQuery)**: Determines if the connection can handle the next request based on the `'Z'` message (ReadyForQuery) sent by the backend.

## 2. Extended Query Protocol

To achieve high performance and security, RBQ uses the PostgreSQL extended query protocol:
1. **Parse**: Sends the SQL statement to the backend for parsing.
2. **Bind**: Binds parameters to the parsed statement, with parameters transmitted in binary format.
3. **Describe**: Retrieves metadata for the result set (column names, types, etc.).
4. **Execute**: Executes the query and retrieves data.
5. **Sync**: Ensures all operations are completed on the backend.

## 3. Error Handling & Mapping

RBQ parses PostgreSQL's `ErrorResponse` packets:
- **Field Extraction**: Extracts `'M'` (message), `'C'` (error code/SQLSTATE), and `'D'` (details).
- **Unified Mapping**: Maps PostgreSQL's rich error information to `RBQError` for precise error classification at higher levels.

## 4. Type System

RBQ can handle PostgreSQL's binary data format:
- **Numeric Types**: Handles `int4`, `int8`, `float8`, etc.
- **Text Types**: Supports `text`, `varchar`, and other UTF-8 encoded data.
- **Extended Types**: Supports advanced types like GIS and JSONB through `DriverExtension`.
