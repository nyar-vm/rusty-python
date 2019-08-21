---
layout: home

hero:
  name: RBQ Engine
  text: Rust Business Query
  tagline: Rust 生态中统一、稳定、高性能的 ORM + RPC 一体化解决方案。
  actions:
    - theme: brand
      text: 开始使用
      link: /guide/introduction
    - theme: alt
      text: 概念指南
      link: /guide/concepts/index
    - theme: alt
      text: 在 GitHub 上查看
      link: https://github.com/rbq-engine/rbq

features:
  - title: ORM + RPC 一体化
    details: 统一数据模型定义和 RPC 服务定义，消除重复编写和更新不一致的痛点。
  - title: 高性能
    details: 零开销抽象，编译期生成最优 Rust 代码，无运行时反射。
  - title: 数据库优先
    details: 每个 .rbq 文件对应一个逻辑数据库，通过 TOML 配置绑定物理数据源。
  - title: 模块化
    details: 通过 using 实现文件间的引用，支持跨文件复用。
  - title: xRPC 实现
    details: 实现 xRPC 概念，支持一元、客户端流、服务端流、双向流等通信模式。
---
