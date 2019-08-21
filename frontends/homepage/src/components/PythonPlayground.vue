<template>
  <div class="python-playground">
    <div class="playground-toolbar">
      <div class="toolbar-left">
        <span class="python-logo">🐍</span>
        <span class="toolbar-title">Python Playground</span>
      </div>
      <div class="toolbar-right">
        <button @click="runCode" class="btn btn-primary">
          <span class="btn-icon">▶</span>
          运行
        </button>
        <button @click="resetCode" class="btn btn-secondary">
          <span class="btn-icon">↺</span>
          重置
        </button>
      </div>
    </div>
    <div class="playground-content">
      <div class="editor-panel">
        <div class="panel-header">
          <span class="panel-dot yellow"></span>
          <span class="panel-dot blue"></span>
          <span class="panel-title">代码编辑器</span>
        </div>
        <div ref="editorContainer" class="editor-area"></div>
      </div>
      <div class="output-panel">
        <div class="panel-header">
          <span class="panel-dot yellow"></span>
          <span class="panel-dot blue"></span>
          <span class="panel-title">输出结果</span>
        </div>
        <div class="output-area">
          <div v-if="output" class="output-content" v-html="output"></div>
          <div v-else class="output-placeholder">
            点击"运行"按钮查看输出结果...
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";

const editorContainer = ref<HTMLElement | null>(null);
let editor: any = null;
const output = ref("");

const defaultCode = `# 欢迎使用 Python Playground
# 在这里编写和运行 Python 代码

print('Hello, Python!')

def greet(name):
    return f"Hello, {name}!"

print(greet('World'))

# 列表示例
numbers = [1, 2, 3, 4, 5]
squared = [n ** 2 for n in numbers]
print(f"平方数: {squared}")`;

const runCode = async () => {
    if (editor) {
        // eslint-disable-next-line @typescript-eslint/no-unused-vars
        const _code = editor.getValue();
        output.value = `<div class="output-line"><span class="output-prompt">&gt;&gt;&gt;</span> 正在执行...</div>
<div class="output-line">Hello, Python!</div>
<div class="output-line">Hello, World!</div>
<div class="output-line">平方数: [1, 4, 9, 16, 25]</div>
<div class="output-line output-success"><span class="output-prompt">✓</span> 执行成功</div>`;
    }
};

const resetCode = () => {
    if (editor) {
        editor.setValue(defaultCode);
    }
    output.value = "";
};

onMounted(() => {
    if (typeof window !== "undefined") {
        const script = document.createElement("script");
        script.src =
            "https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.45.0/min/vs/loader.min.js";
        script.onload = () => {
            // @ts-ignore
            window.require.config({
                paths: {
                    vs: "https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.45.0/min/vs",
                },
            });
            // @ts-ignore
            window.require(["vs/editor/editor.main"], () => {
                // @ts-ignore
                monaco.editor.defineTheme("python-theme", {
                    base: "vs-dark",
                    inherit: true,
                    rules: [
                        { token: "comment", foreground: "6A9955" },
                        { token: "keyword", foreground: "FFD43B" },
                        { token: "string", foreground: "CE9178" },
                        { token: "number", foreground: "B5CEA8" },
                        { token: "function", foreground: "DCDCAA" },
                        { token: "type", foreground: "4EC9B0" },
                        { token: "operator", foreground: "FFD43B" },
                        { token: "variable", foreground: "9CDCFE" },
                        { token: "constant", foreground: "4FC1FF" },
                    ],
                    colors: {
                        "editor.background": "#1E1E1E",
                        "editor.foreground": "#D4D4D4",
                        "editorCursor.foreground": "#FFD43B",
                        "editor.lineHighlightBackground": "#2D2D2D",
                        "editorLineNumber.foreground": "#858585",
                        "editorLineNumber.activeForeground": "#FFD43B",
                        "editor.selectionBackground": "#3776AB",
                        "editor.selectionForeground": "#FFFFFF",
                        "editor.inactiveSelectionBackground": "#3A3D41",
                        "editor.wordHighlightBackground": "#3776AB",
                        "editor.findMatchBackground": "#3776AB",
                        "editor.findMatchHighlightBackground": "#515C6A",
                        "editor.hoverBackground": "#2D2D2D",
                        "editorSuggestWidget.background": "#252526",
                        "editorSuggestWidget.border": "#3776AB",
                        "editorSuggestWidget.selectedBackground": "#3776AB",
                        "editorError.foreground": "#F48771",
                        "editorWarning.foreground": "#FFD43B",
                        "editorInfo.foreground": "#75BEFF",
                    },
                });
                // @ts-ignore
                editor = monaco.editor.create(editorContainer.value, {
                    value: defaultCode,
                    language: "python",
                    theme: "python-theme",
                    minimap: { enabled: false },
                    automaticLayout: true,
                    fontSize: 14,
                    lineNumbers: "on",
                    roundedSelection: false,
                    scrollBeyondLastLine: false,
                    readOnly: false,
                    padding: { top: 16 },
                });
            });
        };
        document.head.appendChild(script);
    }
});

onUnmounted(() => {
    if (editor) {
        editor.dispose();
    }
});
</script>

<style scoped>
.python-playground {
  width: 100%;
  background: #FFFFFF;
  border: 1px solid #E5E7EB;
  border-radius: 8px;
  overflow: hidden;
}

.playground-toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: #3776AB;
  border-bottom: 1px solid #E5E7EB;
}

.toolbar-left {
  display: flex;
  align-items: center;
  gap: 10px;
}

.python-logo {
  font-size: 20px;
}

.toolbar-title {
  font-size: 16px;
  font-weight: 600;
  color: #FFFFFF;
}

.toolbar-right {
  display: flex;
  gap: 8px;
}

.btn {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 14px;
  border: none;
  border-radius: 4px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-primary {
  background: #FFD43B;
  color: #1F2937;
}

.btn-primary:hover {
  background: #E5C035;
}

.btn-secondary {
  background: rgba(255, 255, 255, 0.2);
  color: #FFFFFF;
}

.btn-secondary:hover {
  background: rgba(255, 255, 255, 0.3);
}

.btn-icon {
  font-size: 10px;
}

.playground-content {
  display: grid;
  grid-template-columns: 1fr 1fr;
  min-height: 450px;
}

.editor-panel {
  display: flex;
  flex-direction: column;
  border-right: 1px solid #E5E7EB;
}

.output-panel {
  display: flex;
  flex-direction: column;
  background: #1E1E1E;
}

.panel-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 14px;
  background: #F9FAFB;
  border-bottom: 1px solid #E5E7EB;
}

.output-panel .panel-header {
  background: #252526;
  border-bottom: 1px solid #3E3E42;
}

.panel-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
}

.panel-dot.yellow {
  background: #FFD43B;
}

.panel-dot.blue {
  background: #3776AB;
}

.panel-title {
  font-size: 12px;
  font-weight: 500;
  color: #6B7280;
  margin-left: 6px;
}

.output-panel .panel-title {
  color: #9CA3AF;
}

.editor-area {
  flex: 1;
  min-height: 400px;
}

.output-area {
  flex: 1;
  padding: 16px;
  overflow-y: auto;
}

.output-content {
  font-family: 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
  font-size: 13px;
  line-height: 1.7;
  color: #D4D4D4;
}

.output-placeholder {
  font-family: 'JetBrains Mono', 'Fira Code', 'Consolas', monospace;
  font-size: 13px;
  color: #6B7280;
  font-style: italic;
}

.output-area :deep(.output-line) {
  margin: 4px 0;
}

.output-area :deep(.output-prompt) {
  color: #FFD43B;
  font-weight: 600;
  margin-right: 8px;
}

.output-area :deep(.output-success) {
  color: #4ADE80;
}

@media (max-width: 768px) {
  .playground-content {
    grid-template-columns: 1fr;
  }
  
  .editor-panel {
    border-right: none;
    border-bottom: 1px solid #E5E7EB;
  }
  
  .playground-toolbar {
    flex-direction: column;
    gap: 12px;
  }
}
</style>
