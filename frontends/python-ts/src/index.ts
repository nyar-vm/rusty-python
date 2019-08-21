//! RBQ TypeScript 包装层
//!
//! 此包提供了对 RBQ WebAssembly 功能的 TypeScript 接口。

// 导入 wasm 模块
import * as wasm from "../../../packages/rbq-wasm/pkg/rbq_wasm";

/**
 * 分析 KQL 查询
 *
 * @param query KQL 查询字符串
 * @returns 分析结果的 JSON 字符串
 */
export async function analyzeQuery(query: string): Promise<string> {
    // 确保 wasm 模块已加载
    await wasm.default();
    return wasm.analyze_query(query);
}

/**
 * 生成 SQL 代码
 *
 * @param query KQL 查询字符串
 * @returns 生成的 SQL 代码
 */
export async function generateSql(query: string): Promise<string> {
    // 确保 wasm 模块已加载
    await wasm.default();
    return wasm.generate_sql(query);
}

/**
 * 生成 Rust 代码
 *
 * @param query KQL 查询字符串
 * @returns 生成的 Rust 代码
 */
export async function generateRust(query: string): Promise<string> {
    // 确保 wasm 模块已加载
    await wasm.default();
    return wasm.generate_rust(query);
}
