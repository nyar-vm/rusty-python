//! Python Tools 库
//! 提供 Python 语言相关的工具链

#![warn(missing_docs)]

use oak_core::{Builder, TextEdit, builder::BuildOutput};
use oak_python::{PythonBuilder, PythonLanguage, ast::PythonRoot};

/// Rusty Python 前端
pub struct RustyPythonFrontend {
    language: PythonLanguage,
}

impl Default for RustyPythonFrontend {
    fn default() -> Self {
        Self::new()
    }
}

impl RustyPythonFrontend {
    /// 创建新的前端实例
    pub fn new() -> Self {
        Self { language: PythonLanguage::new() }
    }

    /// 解析 Python 代码
    pub fn parse(&self, source: &str) -> Result<PythonRoot, String> {
        let builder = PythonBuilder::new(&self.language);
        let edits: &[TextEdit] = &[];
        let mut cache = oak_core::parser::ParseSession::default();
        let output: BuildOutput<PythonLanguage> = builder.build(source, edits, &mut cache);
        output.result.map_err(|e| e.to_string())
    }
}
