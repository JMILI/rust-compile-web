<template>
  <div class="explore-container">
    <div class="explore-header">
      <div class="header-left">
        <el-button @click="goBack" icon="el-icon-back" text>
          返回主页
        </el-button>
        <h1>编译过程探索</h1>
      </div>

      <div class="header-right">
        <el-tag type="info" size="large">
          <i class="el-icon-cpu"></i>
          {{ rustVersion }}
        </el-tag>
      </div>
    </div>

    <div class="explore-main">
      <!-- 左侧导航栏 -->
      <div class="explore-sidebar">
        <div class="sidebar-header">
          <h3><i class="el-icon-menu"></i> 编译阶段</h3>
          <el-button
              v-if="allStagesMode"
              type="primary"
              size="small"
              @click="exploreAll"
              :loading="isLoadingAll"
          >
            <i class="el-icon-video-play"></i>
            全部执行
          </el-button>
        </div>

        <div class="stage-list">
          <div
              v-for="stage in availableStages"
              :key="stage.id"
              :class="['stage-item', {
              'active': currentStageId === stage.id,
              'completed': completedStages.includes(stage.id)
            }]"
              @click="selectStage(stage.id)"
          >
            <div class="stage-icon">
              <i v-if="completedStages.includes(stage.id)" class="el-icon-check" />
              <i v-else class="el-icon-document" />
            </div>
            <div class="stage-info">
              <div class="stage-name">{{ stage.name }}</div>
              <div class="stage-id">{{ stage.id }}</div>
            </div>
            <div v-if="loadingStage === stage.id" class="stage-loading">
              <i class="el-icon-loading" />
            </div>
          </div>
        </div>

<!--        <div class="sidebar-options">-->
<!--          <h4><i class="el-icon-setting"></i> 选项</h4>-->
<!--          <div class="options-list">-->
<!--            <el-checkbox v-model="options.verbose" label="详细模式" />-->
<!--            <el-checkbox v-model="options.brief" label="简要模式" />-->
<!--            <el-checkbox v-model="options.fullOutput" label="完整输出" />-->
<!--          </div>-->
<!--        </div>-->

<!--        <div class="sidebar-actions">-->
<!--          <el-button type="primary" @click="copyAllOutputs" :disabled="!hasResults">-->
<!--            <i class="el-icon-copy-document"></i>-->
<!--            复制所有输出-->
<!--          </el-button>-->
<!--          <el-button type="success" @click="exportAllResults" :disabled="!hasResults">-->
<!--            <i class="el-icon-download"></i>-->
<!--            导出结果-->
<!--          </el-button>-->
<!--        </div>-->
      </div>

      <!-- 主内容区域 -->
      <div class="explore-content">
        <div v-if="allStagesMode" class="all-stages-view">
          <div class="stages-header">
            <h2>全部编译阶段结果</h2>
            <div class="stats">
              <el-tag>已完成: {{ completedStages.length }}/{{ availableStages.length }}</el-tag>
              <el-tag type="success" v-if="allCompleted">全部完成</el-tag>
            </div>
          </div>

          <div class="stages-results">
            <el-collapse v-model="activePanels">
              <el-collapse-item
                  v-for="stage in availableStages"
                  :key="stage.id"
                  :name="stage.id"
                  :class="{ 'has-result': stageResults[stage.id] }"
              >
                <template #title>
                  <div class="collapse-header">
                    <div class="header-left">
                      <i v-if="stageResults[stage.id]" class="el-icon-check" style="color: #67c23a; margin-right: 10px;" />
                      <span>{{ stage.name }}</span>
                    </div>
                    <div class="header-right">
                      <el-tag v-if="stageResults[stage.id]" size="small">
                        {{ stageResults[stage.id].lines }} 行
                      </el-tag>
                      <el-button
                          v-if="!stageResults[stage.id]"
                          size="small"
                          type="text"
                          @click.stop="exploreStage(stage.id)"
                          :loading="loadingStage === stage.id"
                      >
                        执行
                      </el-button>
                    </div>
                  </div>
                </template>

                <div v-if="stageResults[stage.id]" class="stage-result">
                  <div class="result-header">
                    <h4>{{ stage.name }}</h4>
                    <div class="result-actions">
                      <el-button size="small" @click="copyStageOutput(stage.id)">复制
                        <i class="el-icon-copy-document"></i>
                      </el-button>
                      <el-button size="small" @click="downloadStageOutput(stage.id)">保存到文档
                        <i class="el-icon-download"></i>
                      </el-button>
                    </div>
                  </div>

                  <div class="result-info">
                    <el-tag size="small">行数: {{ stageResults[stage.id].lines }}</el-tag>
                    <el-tag v-if="stageResults[stage.id].truncated" size="small" type="warning">
                      已截断
                    </el-tag>
                  </div>

                  <div class="result-output">
                    <pre>{{ stageResults[stage.id].output }}</pre>
                  </div>
                </div>

                <div v-else class="stage-empty">
                  <p>尚未执行编译</p>
                  <el-button type="primary" size="small" @click="exploreStage(stage.id)">
                    立即执行
                  </el-button>
                </div>
              </el-collapse-item>
            </el-collapse>
          </div>
        </div>

        <div v-else class="single-stage-view">
          <div class="stage-header">
            <h2>{{ currentStageInfo?.name || '未知阶段' }}</h2>
            <div class="stage-actions">
              <el-button @click="copyOutput" :disabled="!currentResult">
                <i class="el-icon-copy-document"></i> 复制
              </el-button>
              <el-button @click="downloadOutput" :disabled="!currentResult">
                <i class="el-icon-download"></i> 下载
              </el-button>
              <el-button type="primary" @click="exploreStage(currentStageId)" :loading="isLoading">
                <i class="el-icon-refresh"></i> 重新编译
              </el-button>
            </div>
          </div>

          <div class="stage-description">
            <p>{{ currentStageInfo?.description || '' }}</p>
          </div>

          <div v-if="currentError" class="error-section">
            <el-alert
                :title="currentError"
                type="error"
                show-icon
                :closable="true"
                @close="clearError"
            />
          </div>

          <div v-if="currentResult" class="result-section">
            <div class="result-stats">
              <el-tag>行数: {{ currentResult.lines }}</el-tag>
              <el-tag v-if="currentResult.truncated" type="warning">已截断</el-tag>
              <el-tag type="info">阶段: {{ currentResult.stage }}</el-tag>
            </div>

            <div class="result-output">
              <pre>{{ currentResult.output }}</pre>
            </div>
          </div>

          <div v-else class="empty-section">
            <el-empty description="尚未执行编译">
              <el-button type="primary" @click="exploreStage(currentStageId)" :loading="isLoading">
                开始编译
              </el-button>
            </el-empty>
          </div>


        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useCompilerStore } from '@/stores/compiler'
import { ElMessage, ElLoading } from 'element-plus'
import compilerApi from '@/api/compiler'

const route = useRoute()
const router = useRouter()
const compilerStore = useCompilerStore()

// 响应式数据
const code = ref('')
const currentStageId = ref('ast')
const allStagesMode = ref(false)
const loadingStage = ref(null)
const isLoadingAll = ref(false)
const activePanels = ref([])
const stageResults = ref({})
const completedStages = ref([])

// 编译选项
const options = ref({
  verbose: false,
  brief: false,
  fullOutput: false
})

// 计算属性
const availableStages = computed(() => compilerStore.availableStages)
const currentStageInfo = computed(() => {
  return availableStages.value.find(s => s.id === currentStageId.value)
})
const currentResult = computed(() => {
  return stageResults.value[currentStageId.value]
})
const currentError = computed(() => {
  const result = currentResult.value
  return result?.error || null
})
const isLoading = computed(() => {
  return loadingStage.value === currentStageId.value
})
const hasResults = computed(() => {
  return Object.keys(stageResults.value).length > 0
})
const allCompleted = computed(() => {
  return completedStages.value.length === availableStages.value.length
})
const rustVersion = computed(() => {
  return compilerStore.rustVersion?.version || 'Unknown'
})

// 方法
const goBack = () => {
  router.push('/')
}

const selectStage = (stageId) => {
  currentStageId.value = stageId
  if (!allStagesMode.value) {
    loadStageResult(stageId)
  }
}

const exploreStage = async (stageId) => {
  loadingStage.value = stageId
  compilerStore.setStage(stageId)
  compilerStore.updateOptions(options.value)

  try {
    const result = await compilerApi.compileCode(code.value, stageId, options.value)
    stageResults.value[stageId] = result

    if (result.success && !completedStages.value.includes(stageId)) {
      completedStages.value.push(stageId)
    }

    if (result.error) {
      ElMessage.error(`编译 ${stageId} 失败: ${result.error}`)
    } else {
      ElMessage.success(`${currentStageInfo.value?.name || stageId} 编译完成`)
    }
  } catch (error) {
    ElMessage.error(`请求失败: ${error.message}`)
  } finally {
    loadingStage.value = null
  }
}

const exploreAll = async () => {
  isLoadingAll.value = true
  activePanels.value = []

  for (const stage of availableStages.value) {
    await exploreStage(stage.id)
    activePanels.value.push(stage.id)
  }

  isLoadingAll.value = false
  ElMessage.success('所有阶段编译完成')
}

const loadStageResult = async (stageId) => {
  if (!stageResults.value[stageId] && code.value) {
    await exploreStage(stageId)
  }
}

const copyOutput = async () => {
  if (currentResult.value) {
    try {
      await navigator.clipboard.writeText(currentResult.value.output)
      ElMessage.success('输出已复制到剪贴板')
    } catch (err) {
      ElMessage.error('复制失败: ' + err.message)
    }
  }
}

const copyStageOutput = async (stageId) => {
  const result = stageResults.value[stageId]
  if (result) {
    try {
      await navigator.clipboard.writeText(result.output)
      ElMessage.success('输出已复制到剪贴板')
    } catch (err) {
      ElMessage.error('复制失败: ' + err.message)
    }
  }
}

const copyAllOutputs = async () => {
  let allOutput = ''
  for (const stage of availableStages.value) {
    const result = stageResults.value[stage.id]
    if (result) {
      allOutput += `=== ${stage.name} ===\n`
      allOutput += result.output + '\n\n'
    }
  }

  try {
    await navigator.clipboard.writeText(allOutput)
    ElMessage.success('所有输出已复制到剪贴板')
  } catch (err) {
    ElMessage.error('复制失败: ' + err.message)
  }
}

const downloadOutput = () => {
  if (!currentResult.value) return

  const blob = new Blob([currentResult.value.output], { type: 'text/plain' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `rust-compile-${currentStageId.value}-${Date.now()}.txt`
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
  URL.revokeObjectURL(url)
}

const downloadStageOutput = (stageId) => {
  const result = stageResults.value[stageId]
  if (!result) return

  const blob = new Blob([result.output], { type: 'text/plain' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `rust-compile-${stageId}-${Date.now()}.txt`
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
  URL.revokeObjectURL(url)
}

const exportAllResults = () => {
  let exportData = ''

  for (const stage of availableStages.value) {
    const result = stageResults.value[stage.id]
    if (result) {
      exportData += '='.repeat(80) + '\n'
      exportData += `${stage.name}\n`
      exportData += '='.repeat(80) + '\n'
      exportData += `阶段: ${stage.id}\n`
      exportData += `行数: ${result.lines}\n`
      exportData += `Rust版本: ${result.rust_version}\n`
      exportData += '-'.repeat(80) + '\n'
      exportData += result.output + '\n\n'
    }
  }

  const blob = new Blob([exportData], { type: 'text/plain' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `rust-compile-all-stages-${Date.now()}.txt`
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
  URL.revokeObjectURL(url)
}

const clearError = () => {
  if (currentResult.value) {
    currentResult.value.error = null
  }
}

// 初始化
onMounted(async () => {
  await compilerStore.fetchStages()
  await compilerStore.fetchRustVersion()

  // 从路由参数获取代码和阶段
  if (route.query.code) {
    code.value = decodeURIComponent(route.query.code)
    compilerStore.setCode(code.value)
  }

  if (route.query.stage) {
    currentStageId.value = route.query.stage
  }

  if (route.query.all === 'true') {
    allStagesMode.value = true
    activePanels.value = availableStages.value.map(s => s.id)
  }

  // 如果不是全部模式，加载当前阶段
  if (!allStagesMode.value && code.value) {
    await loadStageResult(currentStageId.value)
  }
})

// 监听阶段变化
watch(currentStageId, (newStageId) => {
  if (!allStagesMode.value && code.value && !stageResults.value[newStageId]) {
    loadStageResult(newStageId)
  }
})
</script>

<style scoped>
.explore-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
  display: flex;
  flex-direction: column;
}

.explore-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 30px;
  background: rgba(15, 23, 42, 0.8);
  border-bottom: 1px solid #334155;
  backdrop-filter: blur(10px);
  position: sticky;
  top: 0;
  z-index: 100;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 20px;
}

.header-left h1 {
  margin: 0;
  color: #f8fafc;
  font-size: 1.8rem;
  font-weight: 600;
  background: linear-gradient(90deg, #60a5fa, #3b82f6);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.header-right .el-tag {
  background: rgba(59, 130, 246, 0.1);
  border: 1px solid rgba(59, 130, 246, 0.3);
  color: #60a5fa;
  font-weight: 500;
  padding: 8px 16px;
}

.header-right .el-tag i {
  margin-right: 8px;
}

.explore-main {
  flex: 1;
  display: flex;
  overflow: hidden;
  min-height: calc(100vh - 80px);
}

/* 侧边栏样式 */
.explore-sidebar {
  width: 300px;
  background: rgba(15, 23, 42, 0.8);
  border-right: 1px solid #334155;
  display: flex;
  flex-direction: column;
  overflow-y: auto;
  backdrop-filter: blur(10px);
}

.sidebar-header {
  padding: 20px;
  border-bottom: 1px solid #334155;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.sidebar-header h3 {
  margin: 0;
  color: #e2e8f0;
  font-size: 1.2rem;
  display: flex;
  align-items: center;
  gap: 10px;
}

.stage-list {
  //flex: 1;
  overflow-y: auto;
  padding: 15px;
}

.stage-item {
  display: flex;
  align-items: center;
  padding: 15px;
  cursor: pointer;
  margin-bottom: 10px;
  background: rgba(30, 41, 59, 0.5);
  border: 1px solid transparent;
  border-radius: 10px;
  transition: all 0.2s ease;
}

.stage-item:hover {
  background: rgba(51, 65, 85, 0.7);
  border-color: rgba(96, 165, 250, 0.4);
  transform: translateX(2px);
}

.stage-item.active {
  background: rgba(59, 130, 246, 0.18);
  border-color: #3b82f6;
}

.stage-item.completed {
  border-left: 3px solid #22c55e;
}

.stage-icon {
  width: 34px;
  height: 34px;
  flex-shrink: 0;
  margin-right: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  background: rgba(59, 130, 246, 0.15);
  color: #60a5fa;
}

.stage-item.completed .stage-icon {
  background: rgba(34, 197, 94, 0.15);
  color: #4ade80;
}

.stage-info {
  flex: 1;
  min-width: 0;
}

.stage-name {
  color: #e2e8f0;
  font-size: 0.98rem;
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.stage-id {
  margin-top: 2px;
  color: #64748b;
  font-size: 0.78rem;
  font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
}

.stage-loading {
  margin-left: 8px;
  color: #60a5fa;
}

/* 右侧内容区 */
.explore-content {
  flex: 1;
  min-width: 0;
  padding: 24px 30px 60px;
  overflow-y: auto;
}

.stages-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  margin-bottom: 18px;
}

.stages-header h2,
.stage-header h2 {
  margin: 0;
  color: #f8fafc;
  font-size: 1.35rem;
  font-weight: 600;
}

.stats {
  display: flex;
  gap: 10px;
}

.stages-results :deep(.el-collapse) {
  border: none;
  background: transparent;
}

.stages-results :deep(.el-collapse-item) {
  margin-bottom: 14px;
  border: 1px solid #334155;
  border-radius: 12px;
  overflow: hidden;
  background: rgba(30, 41, 59, 0.55);
}

.stages-results :deep(.el-collapse-item__header) {
  height: 56px;
  padding: 0 18px;
  font-size: 0.98rem;
  color: #e2e8f0;
  background: transparent;
  border-bottom: 1px solid #334155;
}

.stages-results :deep(.el-collapse-item__wrap) {
  background: transparent;
  border-bottom: none;
}

.stages-results :deep(.el-collapse-item__content) {
  padding: 16px 18px 20px;
  color: #cbd5e1;
}

.stages-results :deep(.el-collapse-item.has-result .el-collapse-item__header) {
  border-left: 3px solid #22c55e;
}

.collapse-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  width: 100%;
}

.collapse-header .header-left {
  gap: 6px;
  font-weight: 500;
}

.collapse-header .header-right {
  display: flex;
  align-items: center;
  gap: 10px;
}

.stage-result {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.result-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
}

.result-header h4 {
  margin: 0;
  color: #e2e8f0;
  font-size: 1rem;
}

.result-actions {
  display: flex;
  gap: 8px;
}

.result-info,
.result-stats {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}

.result-output {
  max-height: 60vh;
  padding: 16px;
  overflow: auto;
  border: 1px solid #1e293b;
  border-radius: 10px;
  background: #0b1220;
}

.result-output pre {
  margin: 0;
  color: #a5f3fc;
  font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace;
  font-size: 0.85rem;
  line-height: 1.55;
  white-space: pre-wrap;
  word-break: break-word;
}

.stage-empty,
.empty-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 14px;
  padding: 36px 0;
  color: #94a3b8;
}

/* 单阶段视图 */
.single-stage-view {
  display: flex;
  flex-direction: column;
  gap: 18px;
}

.stage-header {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
}

.stage-actions {
  display: flex;
  gap: 10px;
}

.stage-description {
  padding: 14px 18px;
  color: #94a3b8;
  line-height: 1.7;
  border: 1px solid #334155;
  border-radius: 10px;
  background: rgba(30, 41, 59, 0.45);
}

.stage-description p {
  margin: 0;
}

.error-section {
  margin-top: -4px;
}

.result-section {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

@media (max-width: 900px) {
  .explore-main {
    flex-direction: column;
  }

  .explore-sidebar {
    width: 100%;
    max-height: 40vh;
    border-right: none;
    border-bottom: 1px solid #334155;
  }

  .explore-content {
    padding: 18px;
  }
}
</style>
