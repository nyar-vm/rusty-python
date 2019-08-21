# MySQL Driver Implementation Details

The RBQ MySQL driver aims to provide a high-performance, secure connection solution that strictly adheres to the MySQL Client/Server protocol.

## 1. Authentication Mechanism

RBQ implements MySQL's Authentication Method Switch logic, allowing it to dynamically adjust authentication methods based on server requirements.

### 1.1 mysql_native_password
- **Handling**: When the server sends a `0xFE` packet requesting a switch, RBQ checks the plugin name.
- **Encryption Logic**: Uses SHA1 double-hashing (Scramble mechanism).
  1. `SHA1(password)`
  2. `SHA1(SHA1(password))`
  3. `SHA1(scramble + SHA1(SHA1(password))) ^ SHA1(password)`
- **Security**: Rejects plaintext password transmission, ensuring password protection even over non-SSL connections.

## 2. Parameterized Queries & Security

RBQ enforces the use of the MySQL binary protocol for data interaction to completely prevent SQL injection.

- **Prepare Phase**: Uses the `COM_STMT_PREPARE` command to send the SQL template.
- **Execute Phase**: Uses the `COM_STMT_EXECUTE` command to send parameters in binary format.
- **Data Mapping**: Automatically converts Rust types (e.g., `i32`, `String`, `f64`) into the MySQL binary protocol format.

## 3. Packet Parsing

RBQ implements a lightweight packet parsing engine:
- **Streaming**: Processes MySQL's 4-byte packet header via the `read_packet` function.
- **State Machine**: Parses packets for different phases like Handshake, AuthSwitch, OK, ERR, and ResultSet.
- **Error Handling**: Maps MySQL error codes to a unified `RBQError::DatabaseError`.
