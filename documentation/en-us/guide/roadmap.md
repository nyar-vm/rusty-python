# Roadmap

This roadmap is based on the [Technical Whitepaper](./architecture.md) and guides the phased development and delivery of the RBQ engine.

## 1. Phase 1: Foundation (M1 - M3) [Completed]
**Goal**: Establish core abstraction layers and implement basic driver integration for mainstream databases.

### 1.1 Core Milestones
- **M1: Core Interface Definition Complete** [Completed]
  - Define core traits like `Connection`, `ResultSet`, `Value`.
  - Establish `RBQError` unified error handling system.
- **M2: First Driver Adapter Passed Tests** [Completed]
  - Complete basic implementation of `we-trust-postgres` (PostgreSQL).
  - Implement basic parameterized queries and type conversions.
- **M3: Basic Performance Benchmarks Established** [In Progress]
  - Integrate `criterion` for performance pressure testing.
  - Establish contrast benchmarks with native drivers.

## 2. Phase 2: Feature Enhancement (M4 - M6) [Completed]
**Goal**: Expand database support, refine connection pooling, and observability systems.

### 2.1 Core Milestones
- **M4: Multi-Database Driver Integration** [Completed]
  - Integrated `we-trust-mysql` and `we-trust-sqlite`.
  - **Todo**: Integration of `we-trust-sqlserver`.
- **M5: Advanced Connection Pool Features** [Completed]
  - Implemented dynamic scaling, health checks, idle connection cleanup, and background maintenance tasks.
- **M6: Observability System** [Completed]
  - Full integration with `tracing` for structured logging and distributed tracing.
  - Extended `PoolMetrics` for comprehensive monitoring.

## 3. Phase 3: Production Readiness (M7 - M9) [In Progress]
**Goal**: Ultimate performance optimization, security auditing, and official 1.0 release.

### 3.1 Core Milestones
- **M7: Security Hardening & Audit** [Completed]
  - **SQL Injection Defense**: Implemented binary protocol for MySQL and extended query protocol for PostgreSQL.
  - **Transport Security**: Integrated full TLS/SSL support across drivers.
- **M8: Production Performance Targets** [In Progress]
  - Complete zero-copy optimizations, ensuring RBQ abstraction overhead < 5% of native drivers.
  - Establish performance benchmarks against native drivers.
- **M9: Official 1.0 Release** [Planned]
  - Achieve 100% documentation coverage.
  - Create production integration guides and case studies.
