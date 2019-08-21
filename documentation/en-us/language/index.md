# RBQ Language Guide

Welcome to the learning journey of RBQ language. This guide will take you from scratch, step by step, to master this declarative modeling and query language designed specifically for modern Rust backends.

We recommend you read in the following order:

## Phase 1: Core Foundation
Understand the essence of RBQ and master the most basic data expression methods.

1.  **[Design Philosophy](philosophy.md)**：Why do we need RBQ? Understand the "cyber-commerce" aesthetics and three-layer IR architecture.
2.  **[Quick Start: Basic Syntax](basics.md)**：Comments, basic types, and how to define your first class.
3.  **[Entity Modeling: @table and Primary Key](entity-modeling.md)**：Understand the difference between entity tables and free objects, master primary keys and indexes.
4.  **[Algebraic Data Types: Unions and ADT](unions.md)**：Go beyond simple unions, learn about parameterized ADTs and their physical representation.
5.  **[Relationships, Foreign Keys, and Preloading](relations.md)**：Understand `&T` (physical foreign key) and `@relation` (logical association), solve the N+1 problem.

## Phase 2: Functional Query DSL
Now that data is defined, learn how to efficiently query and transform it.

6.  **[Data Flow Thinking and Basic Queries](query-basics.md)**：Understand `filter` pipelines and basic comparison operations.
7.  **[Projection and Transformation](projection.md)**：Use `map` to shape data, master the `$field` shorthand syntax.
8.  **[Aggregation and Statistics](aggregation.md)**：Advanced data analysis such as grouping, counting, and summing.

## Phase 3: Engineering Organization and Security
How to maintain code maintainability and security when projects grow larger and more complex.

9.  **[Field Reuse: Traits and Mixins](traits.md)**：Use `trait` and `using` to reduce template code and implement cross-cutting concerns.
10. **[Multi-database Mixing and Namespaces](namespaces.md)**：Use `namespace` to organize large projects, master PostgreSQL Schema flattening rules.
11. **[Security and Isolation: Policies and Tenants](security.md)**：Use `@policy` to implement row-level security, utilize `@tenant` for native isolation.
12. **[Engineering Enhancements: Built-in Magic](engineering.md)**：Master engineering annotations such as `@audit`, `@soft_delete`, `@version`.

## Phase 4: Modern Paradigms and Extensions
Explore the boundaries of RBQ, handle unstructured data and custom logic.

13. **[Logic Encapsulation: Micro Functions](micro-functions.md)**：Define compile-time inline pure logic operators.
14. **[Modern Data Paradigms: JSON and Vectors](modern-paradigms.md)**：Document schema and AI vector search integration.
15. **[WASI UDF Extensions](udf-wasi.md)**：Extend database capabilities using WebAssembly component model.

## Advanced: Operations and Underlying Mechanisms
After mastering the modeling language, you may need to delve into how to manage the database lifecycle or its operating mechanism.

- **[RBQ CLI Tool Guide](../guide/commands/index.md)**：Migration, backup, and production deployment.
  - **[Maintenance Guide](../maintenance/index.md)**：Deeply understand compiler IR, driver adapters, and internal mechanisms.