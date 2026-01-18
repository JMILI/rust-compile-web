<template>
  <div class="home-container">
    <header class="header">
      <h1><i class="el-icon-cpu"></i> Rust 编译过程探索工具</h1>
      <p class="subtitle">探索 Rust 代码从源代码到机器码的完整编译过程</p>

      <div v-if="rustVersion" class="rust-info">
        <el-tag :type="rustVersion.is_nightly ? 'warning' : 'info'">
          <i class="el-icon-sunrise"></i>
          {{ rustVersion.version }}
          <span v-if="!rustVersion.is_nightly" style="color: #f56c6c; margin-left: 5px;">
            (建议使用 nightly 版本)
          </span>
        </el-tag>
      </div>
    </header>

    <main class="main-content">
      <div class="code-editor-section">
        <div class="section-header">
          <h2><i class="el-icon-edit"></i> 输入 Rust 代码</h2>
          <div class="actions">
            <el-button type="primary" @click="useExample">
              <i class="el-icon-collection"></i> 使用示例
            </el-button>
            <el-button @click="clearCode">
              <i class="el-icon-delete"></i> 清空
            </el-button>
          </div>
        </div>

        <div class="code-editor">
          <el-input
              v-model="code"
              type="textarea"
              :rows="15"
              placeholder="请输入 Rust 代码..."
              resize="none"
              class="code-input"
          />

          <div class="code-info">
            <span>行数: {{ lineCount }}</span>
            <span>字符数: {{ charCount }}</span>
          </div>
        </div>

        <div class="stage-selection">
          <h3><i class="el-icon-s-operation"></i> 选择编译阶段</h3>
          <div class="stage-grid">
            <el-card
                v-for="stage in availableStages"
                :key="stage.id"
                :class="{ 'active': currentStage === stage.id }"
                @click="selectStage(stage.id)"
                shadow="hover"
                class="stage-card"
            >
              <div class="stage-content">
                <h4>{{ stage.name }}</h4>
                <p class="stage-desc">{{ stage.description }}</p>
                <el-tag v-if="currentStage === stage.id" type="success">
                  <i class="el-icon-check"></i> 已选择
                </el-tag>
              </div>
            </el-card>
          </div>
        </div>

<!--        <div class="options-section">-->
<!--          <h3><i class="el-icon-setting"></i> 编译选项</h3>-->
<!--          <div class="options-grid">-->
<!--            <el-checkbox v-model="options.verbose" label="详细模式" />-->
<!--            <el-checkbox v-model="options.brief" label="简要模式" />-->
<!--            <el-checkbox v-model="options.fullOutput" label="完整输出" />-->
<!--          </div>-->
<!--        </div>-->

        <div class="action-section">
          <el-button
              type="primary"
              size="large"
              :loading="isLoading"
              :disabled="!code.trim()"
              @click="exploreCompile"
              class="explore-button"
          >
            <i class="el-icon-search"></i>
            {{ isLoading ? '编译中...' : '开始探索' }}
          </el-button>

          <el-button
              type="success"
              size="large"
              :disabled="!code.trim()"
              @click="exploreAllStages"
          >
            <i class="el-icon-s-data"></i>
            探索所有阶段
          </el-button>
        </div>

        <div v-if="error" class="error-message">
          <el-alert
              :title="error"
              type="error"
              show-icon
              :closable="true"
              @close="clearError"
          />
        </div>
      </div>

      <div v-if="hasResult" class="quick-preview">
        <div class="section-header">
          <h2><i class="el-icon-view"></i> 快速预览</h2>
          <el-button @click="goToExplore">
            <i class="el-icon-right"></i> 查看详情
          </el-button>
        </div>

        <el-card class="preview-card">
          <div class="preview-header">
            <h3>{{ compileResult.stage_name }}</h3>
            <div class="preview-stats">
              <el-tag>{{ compileResult.lines }} 行</el-tag>
              <el-tag v-if="compileResult.truncated" type="warning">已截断</el-tag>
            </div>
          </div>

          <div class="preview-content">
            <pre class="output-preview">{{ truncatedOutput }}</pre>
          </div>

          <div class="preview-footer">
            <span class="rust-version">
              <i class="el-icon-info"></i> Rust: {{ compileResult.rust_version }}
            </span>
            <el-button type="text" @click="copyOutput">
              <i class="el-icon-copy-document"></i> 复制输出
            </el-button>
          </div>
        </el-card>
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useCompilerStore } from '@/stores/compiler'
import { ElMessage } from 'element-plus'

const router = useRouter()
const compilerStore = useCompilerStore()

// 响应式数据
const code = computed({
  get: () => compilerStore.code,
  set: (value) => compilerStore.setCode(value)
})

const currentStage = computed({
  get: () => compilerStore.currentStage,
  set: (value) => compilerStore.setStage(value)
})

const options = computed({
  get: () => compilerStore.compileOptions,
  set: (value) => compilerStore.updateOptions(value)
})

// 计算属性
const lineCount = computed(() => {
  return code.value.split('\n').length
})

const charCount = computed(() => {
  return code.value.length
})

const availableStages = computed(() => compilerStore.availableStages)
const isLoading = computed(() => compilerStore.isLoading)
const error = computed(() => compilerStore.error)
const compileResult = computed(() => compilerStore.compileResult)
const hasResult = computed(() => compilerStore.hasResult)
const rustVersion = computed(() => compilerStore.rustVersion)

// 截断预览输出
const truncatedOutput = computed(() => {
  if (!compileResult.value) return ''
  const output = compileResult.value.output
  if (output.length > 500) {
    return output.substring(0, 500) + '\n... (输出已截断，点击"查看详情"查看完整内容)'
  }
  return output
})

// 示例代码
const exampleCode = `// 简单的Rust程序用于编译探索
#[inline(never)]
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let a = 10;
    let b = 20;
    let sum = add(a, b);
    println!("结果: {}", sum);
}`

// 方法
const useExample = () => {
  code.value = exampleCode
}

const clearCode = () => {
  code.value = ''
  compilerStore.compileResult = null
  compilerStore.error = null
}

const selectStage = (stageId) => {
  currentStage.value = stageId
}

const clearError = () => {
  compilerStore.error = null
}

const exploreCompile = async () => {
  await compilerStore.compile()
}

const exploreAllStages = () => {
  if (code.value.trim()) {
    router.push({
      path: '/explore',
      query: {
        code: encodeURIComponent(code.value),
        all: 'true'
      }
    })
  }
}

const goToExplore = () => {
  if (code.value.trim() && currentStage.value) {
    router.push({
      path: '/explore',
      query: {
        code: encodeURIComponent(code.value),
        stage: currentStage.value
      }
    })
  }
}

const copyOutput = async () => {
  if (compileResult.value) {
    try {
      await navigator.clipboard.writeText(compileResult.value.output)
      ElMessage.success('输出已复制到剪贴板')
    } catch (err) {
      ElMessage.error('复制失败: ' + err.message)
    }
  }
}

// 生命周期
onMounted(async () => {
  await compilerStore.fetchStages()
  await compilerStore.fetchRustVersion()
})
</script>

<style scoped>
/* 深色主题变量 */
:root {
  --primary-color: #409eff;
  --primary-light: #ecf5ff;
  --success-color: #67c23a;
  --warning-color: #e6a23c;
  --danger-color: #f56c6c;
  --info-color: #909399;
  --text-primary: #303133;
  --text-regular: #606266;
  --text-secondary: #909399;
  --border-color: #fff;
  --background-base: #0d1117;
  --background-card: #161b22;
  --background-hover: #21262d;
  --border-dark: #30363d;
  --code-bg: #0d1117;
  --code-text: #ef1b1b;
}

.home-container {
  max-width: 1400px;
  margin: 0 auto;
  padding: 20px;
  min-height: 100vh;
  background-color: var(--background-base);
}

.header {
  text-align: center;
  margin-bottom: 40px;
  padding: 30px;
  background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
  border-radius: 16px;
  color: white;
  border: 1px solid var(--border-dark);
  position: relative;
  overflow: hidden;
}

.header::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" preserveAspectRatio="none"><path d="M0,0 L100,0 L100,100 Z" fill="rgba(255,255,255,0.05)"/></svg>');
  background-size: cover;
}

.header h1 {
  font-size: 2.8rem;
  margin-bottom: 15px;
  background: linear-gradient(90deg, #60a5fa, #3b82f6);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  font-weight: 700;
  position: relative;
}

.header h1 i {
  margin-right: 15px;
  font-size: 2.5rem;
  vertical-align: middle;
}

.subtitle {
  font-size: 1.3rem;
  opacity: 0.9;
  margin-bottom: 25px;
  color: #cbd5e1;
  max-width: 800px;
  margin-left: auto;
  margin-right: auto;
  line-height: 1.6;
}

.rust-info {
  display: inline-block;
  margin-top: 15px;
}

.main-content {
  display: grid;
  gap: 30px;
}

.code-editor-section {
  background: var(--background-card);
  padding: 30px;
  border-radius: 16px;
  border: 1px solid var(--border-dark);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
  transition: all 0.3s ease;
}

.code-editor-section:hover {
  box-shadow: 0 12px 48px rgba(0, 0, 0, 0.4);
  transform: translateY(-2px);
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 25px;
  padding-bottom: 15px;
  border-bottom: 1px solid var(--border-dark);
}

.section-header h2 {
  margin: 0;
  color: #e2e8f0;
  font-size: 1.5rem;
  display: flex;
  align-items: center;
  gap: 10px;
}

.section-header h2 i {
  color: var(--primary-color);
}

.actions {
  display: flex;
  gap: 10px;
}

.code-editor {
  margin-bottom: 30px;
  position: relative;
}

.code-input {
  font-family: 'JetBrains Mono', 'Fira Code', 'Cascadia Code', 'Monaco', 'Menlo', monospace;
  font-size: 14px;
  line-height: 1.6;
  background: var(--code-bg);
  border: 1px solid var(--border-dark);
  border-radius: 10px;
  color: #fff;
  transition: all 0.3s ease;
}

.code-input:deep(.el-textarea__inner) {
  background: var(--code-bg);
  color: #fff;
  border: 1px solid var(--border-dark);
  border-radius: 10px;
  padding: 20px;
  transition: all 0.3s ease;
  resize: vertical;
  min-height: 300px;
  font-family: 'JetBrains Mono', 'Fira Code', 'Cascadia Code', 'Monaco', 'Menlo', monospace;
  font-size: 14px;
  line-height: 1.6;
}

.code-input:deep(.el-textarea__inner):focus {
  border-color: var(--primary-color);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.2);
}

.code-info {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 10px;
  padding: 0 5px;
}

.code-info span {
  color: var(--text-secondary);
  font-size: 0.9rem;
  background: rgba(99, 102, 241, 0.1);
  padding: 4px 12px;
  border-radius: 20px;
  border: 1px solid rgba(99, 102, 241, 0.2);
}

.stage-selection {
  margin-bottom: 30px;
}

.stage-selection h3 {
  color: #e2e8f0;
  margin-bottom: 20px;
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 1.3rem;
}

.stage-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 20px;
  margin-top: 20px;
}

.stage-card {
  background: linear-gradient(145deg, #1a1f2e, #161b22);
  border: 1px solid var(--border-dark);
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  overflow: hidden;
}

.stage-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  width: 4px;
  height: 100%;
  background: linear-gradient(to bottom, var(--primary-color), #3b82f6);
  transform: scaleY(0);
  transition: transform 0.3s ease;
}

.stage-card.active {
  border-color: var(--primary-color);
  box-shadow: 0 4px 20px rgba(59, 130, 246, 0.2);
  transform: translateY(-4px);
}

.stage-card.active::before {
  transform: scaleY(1);
}

.stage-card:hover {
  transform: translateY(-6px);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
}

.stage-card:deep(.el-card__body) {
  padding: 20px;
}

.stage-content h4 {
  margin: 0 0 10px 0;
  color: #f1f5f9;
  font-size: 1.1rem;
  font-weight: 600;
}

.stage-desc {
  color: #94a3b8;
  font-size: 0.9rem;
  line-height: 1.5;
  margin-bottom: 15px;
  min-height: 60px;
}

.options-section {
  margin-bottom: 30px;
  background: rgba(30, 41, 59, 0.5);
  padding: 20px;
  border-radius: 12px;
  border: 1px solid var(--border-dark);
}

.options-section h3 {
  color: #e2e8f0;
  margin-bottom: 15px;
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 1.3rem;
}

.options-grid {
  display: flex;
  gap: 30px;
  flex-wrap: wrap;
}

.options-grid :deep(.el-checkbox) {
  margin-right: 0;
}

.options-grid :deep(.el-checkbox__label) {
  color: #cbd5e1;
}

.options-grid :deep(.el-checkbox__inner) {
  background-color: #1e293b;
  border-color: #475569;
}

.action-section {
  display: flex;
  gap: 20px;
  justify-content: center;
  margin: 30px 0;
}

.explore-button {
  padding: 0 40px;
  font-size: 1.1rem;
  height: 48px;
  background: linear-gradient(90deg, #3b82f6, #2563eb);
  border: none;
  border-radius: 12px;
  font-weight: 600;
  transition: all 0.3s ease;
}

.explore-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(59, 130, 246, 0.4);
}

.error-message {
  margin-top: 20px;
}

.error-message :deep(.el-alert) {
  background: rgba(239, 68, 68, 0.1);
  border: 1px solid rgba(239, 68, 68, 0.2);
  border-radius: 10px;
}

.quick-preview {
  background: var(--background-card);
  padding: 30px;
  border-radius: 16px;
  border: 1px solid var(--border-dark);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
}

.preview-card {
  background: linear-gradient(145deg, #1a1f2e, #161b22);
  border: 1px solid var(--border-dark);
  border-radius: 12px;
  overflow: hidden;
}

.preview-card:deep(.el-card__body) {
  padding: 0;
}

.preview-header {
  padding: 20px;
  border-bottom: 1px solid var(--border-dark);
  background: rgba(15, 23, 42, 0.5);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.preview-header h3 {
  margin: 0;
  color: #f1f5f9;
  font-size: 1.2rem;
  display: flex;
  align-items: center;
  gap: 10px;
}

.preview-stats {
  display: flex;
  gap: 10px;
}

.preview-content {
  padding: 20px;
  background: var(--code-bg);
}

.output-preview {
  margin: 0;
  font-family: 'JetBrains Mono', 'Fira Code', 'Cascadia Code', monospace;
  font-size: 13px;
  line-height: 1.5;
  white-space: pre-wrap;
  word-break: break-all;
  color: #c9d1d9;
  max-height: 400px;
  overflow-y: auto;
  padding: 10px;
  background: #0d1117;
  border-radius: 8px;
  border: 1px solid var(--border-dark);
}

.output-preview::-webkit-scrollbar {
  width: 8px;
}

.output-preview::-webkit-scrollbar-track {
  background: #161b22;
  border-radius: 4px;
}

.output-preview::-webkit-scrollbar-thumb {
  background: #475569;
  border-radius: 4px;
}

.output-preview::-webkit-scrollbar-thumb:hover {
  background: #64748b;
}

.preview-footer {
  padding: 15px 20px;
  border-top: 1px solid var(--border-dark);
  background: rgba(15, 23, 42, 0.5);
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.rust-version {
  color: #94a3b8;
  font-size: 0.9rem;
  display: flex;
  align-items: center;
  gap: 8px;
}

/* 响应式设计 */
@media (max-width: 1200px) {
  .stage-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 768px) {
  .home-container {
    padding: 15px;
  }

  .header {
    padding: 20px;
  }

  .header h1 {
    font-size: 2rem;
  }

  .stage-grid {
    grid-template-columns: 1fr;
  }

  .options-grid {
    flex-direction: column;
    gap: 15px;
  }

  .action-section {
    flex-direction: column;
  }

  .explore-button {
    width: 100%;
  }

  .section-header {
    flex-direction: column;
    gap: 15px;
    align-items: flex-start;
  }

  .actions {
    width: 100%;
    justify-content: flex-start;
  }
}
</style>