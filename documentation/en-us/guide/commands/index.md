# RBQ CLI Tool Guide

RBQ provides a powerful Command Line Interface (CLI) tool for managing the database lifecycle, including initialization, migration, backup, recovery, and production environment deployment.

## Core Command Overview

| Command | Description | Application Scenario |
| :--- | :--- | :--- |
| **[rbq check](check.md)** | Check .rbq file syntax and semantic errors | Development phase verification |
| **[rbq compile](compile.md)** | Compile .rbq files into target code (Rust/TS) | Code generation |
| **[rbq generate](generate.md)** | Generate queries or documentation | Assisted development |
| **[rbq migration](migration.md)** | Database migration management (create, up, down, status) | Structure change synchronization |
| **[rbq pull](pull.md)** | Pull Schema from existing database | Reverse engineering |
| **[rbq push](push.md)** | Push model definitions to database | Structure change application |
| **[rbq backup](backup.md)** | Database physical or logical backup | Data security guarantee |
| `rbq restore` | Restore data from backup | Disaster recovery |

---

## Quick Navigation

1. **[Syntax Check (Check)](check.md)**：Validate the legality of model definitions.
2. **[Code Compilation (Compile)](compile.md)**：Convert models to strongly typed code.
3. **[Database Migration (Migration)](migration.md)**：How to safely manage Schema changes.
4. **[Schema Pull (Pull)](pull.md)**：Reverse generate models from existing databases.
5. **[Schema Push (Push)](push.md)**：Apply model definitions to databases.
6. **[Backup & Restore](backup.md)**：Cross-engine data protection solution.
7. **[Production Environment Deployment](deployment.md)**：CI/CD integration and best practices.

---

## Related Resources

- **[Concepts Guide](../concepts/index.md)**：Understand core concepts such as ORM, RPC, xRPC.
- **[RBQ Language Guide](../../language/index.md)**：Learn the syntax and features of RBQ DSL.
- **[Architecture Guide](../architecture.md)**：Deeply understand RBQ's architecture design.