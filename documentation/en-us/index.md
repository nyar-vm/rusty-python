---
layout: home

hero:
  name: RBQ Engine
  text: Rust Business Query
  tagline: Unified, stable, and high-performance ORM + RPC integrated solution for Rust.
  actions:
    - theme: brand
      text: Get Started
      link: /en/guide/introduction
    - theme: alt
      text: Concepts Guide
      link: /en/guide/concepts/index
    - theme: alt
      text: View on GitHub
      link: https://github.com/rbq-engine/rbq

features:
  - title: ORM + RPC Integration
    details: Unified data model definition and RPC service definition, eliminating duplicate code and inconsistent updates.
  - title: High Performance
    details: Zero-overhead abstraction, compile-time generation of optimal Rust code, no runtime reflection.
  - title: Database First
    details: Each .rbq file corresponds to a logical database, bound to physical data sources through TOML configuration.
  - title: Modular
    details: Reference between files via using, supporting cross-file reuse.
  - title: xRPC Implementation
    details: Implements xRPC concept, supporting unary, client streaming, server streaming, bidirectional streaming communication modes.
---
