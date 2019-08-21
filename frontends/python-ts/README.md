# RBQ TypeScript Frontend

RBQ TypeScript 前端包，负责接受 WASM 并导出功能。

## 功能特性

- **WASM 集成**：接受并集成 RBQ WASM 模块
- **TypeScript 封装**：为 WASM 功能提供 TypeScript 封装
- **功能导出**：将 RBQ 核心功能导出为 TypeScript API
- **类型安全**：提供完整的 TypeScript 类型定义

## 安装

```bash
# 使用 npm
npm install rbq-ts

# 使用 yarn
yarn add rbq-ts

# 使用 pnpm
pnpm add rbq-ts
```

## 使用示例

```typescript
import { RBQ } from 'rbq-ts';

async function main() {
  // 初始化 RBQ
  const rbq = await RBQ.init();
  
  // 使用 RBQ 功能
  // ...
}

main().catch(console.error);
```

## 架构

- **WASM 加载**：负责加载和初始化 RBQ WASM 模块
- **API 封装**：为 WASM 功能提供 TypeScript API
- **类型定义**：提供完整的 TypeScript 类型支持

## 依赖

- 无外部依赖

## 开发

```bash
# 构建
pnpm build

# 开发模式
pnpm dev
```

## 许可证

MIT 或 Apache-2.0

## 贡献

欢迎提交 issue 和 PR 来改进这个项目！