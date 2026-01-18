<template>
  <div class="ast-tree-view">
    <!-- 顶部工具栏 -->
    <div class="ast-toolbar">
      <div class="toolbar-left">
        <h3><i class="el-icon-data-analysis"></i> 抽象语法树可视化</h3>
        <div class="stats">
          <el-tag size="small" type="info">
            <i class="el-icon-document"></i> {{ stats.nodeCount }} 个节点
          </el-tag>
          <el-tag size="small" type="info">
            <i class="el-icon-sort"></i> {{ stats.indentLevels }} 层深度
          </el-tag>
          <el-tag size="small" type="info">
            <i class="el-icon-tickets"></i> {{ stats.nonEmptyLines }} 行
          </el-tag>
        </div>
      </div>

      <div class="toolbar-right">
        <el-button-group>
          <el-button size="small" @click="changeLayout('compactBox')" :type="layout === 'compactBox' ? 'primary' : ''">
            <i class="el-icon-s-grid"></i> 紧凑布局
          </el-button>
          <el-button size="small" @click="changeLayout('dendrogram')" :type="layout === 'dendrogram' ? 'primary' : ''">
            <i class="el-icon-s-operation"></i> 树状布局
          </el-button>
          <el-button size="small" @click="changeLayout('mindmap')" :type="layout === 'mindmap' ? 'primary' : ''">
            <i class="el-icon-map-location"></i> 脑图布局
          </el-button>
        </el-button-group>

        <el-dropdown @command="handleCommand" class="layout-dropdown">
          <el-button size="small">
            <i class="el-icon-setting"></i> 选项
          </el-button>
          <template #dropdown>
            <el-dropdown-menu>
              <el-dropdown-item command="toggleLabels">
                <i :class="['el-icon', showLabels ? 'el-icon-view' : 'el-icon-hide']"></i>
                {{ showLabels ? '隐藏标签' : '显示标签' }}
              </el-dropdown-item>
              <el-dropdown-item command="zoomIn">
                <i class="el-icon-zoom-in"></i> 放大
              </el-dropdown-item>
              <el-dropdown-item command="zoomOut">
                <i class="el-icon-zoom-out"></i> 缩小
              </el-dropdown-item>
              <el-dropdown-item command="fitView">
                <i class="el-icon-full-screen"></i> 适应视图
              </el-dropdown-item>
              <el-dropdown-item divided command="downloadImage">
                <i class="el-icon-download"></i> 保存图片
              </el-dropdown-item>
            </el-dropdown-menu>
          </template>
        </el-dropdown>
      </div>
    </div>

    <!-- 主要内容区域 -->
    <div class="ast-main">
      <!-- 左侧：树状图可视化 -->
      <div class="tree-container">
        <div class="graph-header">
          <h4>树状结构可视化</h4>
          <el-button v-if="isLoading" :loading="true" size="mini">加载中...</el-button>
        </div>
        <div ref="graphContainer" class="graph-wrapper"></div>

        <div class="graph-legend">
          <div class="legend-item">
            <span class="legend-color" style="background-color: #722ed1;"></span>
            <span>模块/包 (Crate)</span>
          </div>
          <div class="legend-item">
            <span class="legend-color" style="background-color: #13c2c2;"></span>
            <span>函数/项 (Fn/Item)</span>
          </div>
          <div class="legend-item">
            <span class="legend-color" style="background-color: #52c41a;"></span>
            <span>代码块 (Block)</span>
          </div>
          <div class="legend-item">
            <span class="legend-color" style="background-color: #fa8c16;"></span>
            <span>语句/表达式 (Stmt/Expr)</span>
          </div>
          <div class="legend-item">
            <span class="legend-color" style="background-color: #f5222d;"></span>
            <span>字面量 (Literal)</span>
          </div>
        </div>
      </div>

      <!-- 右侧：详细信息面板 -->
      <div class="info-panel">
        <div class="panel-header">
          <h4>AST 详细信息</h4>
          <el-button size="mini" @click="togglePanel">
            <i :class="['el-icon', panelExpanded ? 'el-icon-arrow-right' : 'el-icon-arrow-left']"></i>
          </el-button>
        </div>

        <div v-if="panelExpanded" class="panel-content">
          <!-- 节点详细信息 -->
          <div v-if="selectedNode" class="node-details">
            <h5>当前节点</h5>
            <div class="detail-item">
              <span class="detail-label">类型:</span>
              <span class="detail-value">{{ selectedNode.type || '未知' }}</span>
            </div>
            <div class="detail-item">
              <span class="detail-label">标签:</span>
              <span class="detail-value">{{ selectedNode.label }}</span>
            </div>
            <div v-if="selectedNode.depth !== undefined" class="detail-item">
              <span class="detail-label">深度:</span>
              <span class="detail-value">第 {{ selectedNode.depth }} 层</span>
            </div>
            <div v-if="selectedNode.fullText" class="detail-item full">
              <span class="detail-label">原始内容:</span>
              <pre class="detail-value">{{ selectedNode.fullText }}</pre>
            </div>
          </div>

          <!-- AST 统计信息 -->
          <div class="stats-section">
            <h5>关键信息提取</h5>
            <div v-if="astInfo.functions.length > 0" class="info-section">
              <h6><i class="el-icon-c-scale-to-original"></i> 函数定义</h6>
              <div class="info-items">
                <el-tag
                    v-for="(fn, idx) in astInfo.functions"
                    :key="`fn-${idx}`"
                    size="small"
                    type="success"
                    class="info-tag"
                >
                  {{ fn }}
                </el-tag>
              </div>
            </div>

            <div v-if="astInfo.variables.length > 0" class="info-section">
              <h6><i class="el-icon-s-flag"></i> 变量声明</h6>
              <div class="info-items">
                <el-tag
                    v-for="(varName, idx) in astInfo.variables"
                    :key="`var-${idx}`"
                    size="small"
                    class="info-tag"
                >
                  {{ varName }}
                </el-tag>
              </div>
            </div>

            <div v-if="astInfo.literals.length > 0" class="info-section">
              <h6><i class="el-icon-coin"></i> 字面量</h6>
              <div class="info-items">
                <el-tag
                    v-for="(lit, idx) in astInfo.literals"
                    :key="`lit-${idx}`"
                    size="small"
                    type="warning"
                    class="info-tag"
                >
                  "{{ lit }}"
                </el-tag>
              </div>
            </div>

            <div v-if="astInfo.structures.length > 0" class="info-section">
              <h6><i class="el-icon-s-grid"></i> 结构体/枚举</h6>
              <div class="info-items">
                <el-tag
                    v-for="(struct, idx) in astInfo.structures"
                    :key="`struct-${idx}`"
                    size="small"
                    type="danger"
                    class="info-tag"
                >
                  {{ struct }}
                </el-tag>
              </div>
            </div>
          </div>

          <!-- 操作按钮 -->
          <div class="action-buttons">
            <el-button size="small" @click="exportJson" :disabled="!astTree">
              <i class="el-icon-document"></i> 导出 JSON
            </el-button>
            <el-button size="small" @click="copyAst" :disabled="!astText">
              <i class="el-icon-copy-document"></i> 复制 AST
            </el-button>
          </div>
        </div>
      </div>
    </div>

    <!-- 原始文本展示（可折叠） -->
    <div class="raw-ast-section">
      <div class="section-header" @click="showRawAst = !showRawAst">
        <h4>
          <i :class="['el-icon', showRawAst ? 'el-icon-arrow-down' : 'el-icon-arrow-right']"></i>
          原始 AST 文本
        </h4>
        <el-tag size="small" type="info">
          {{ astText.length }} 字符
        </el-tag>
      </div>

      <div v-if="showRawAst" class="raw-ast-content">
        <pre class="ast-text">{{ astText }}</pre>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, watch, nextTick } from 'vue'
import { parseAstToTree, convertToGraphData, extractAstInfo, getAstStats, simplifyAstTree } from '@/utils/ast-parser'
import { ElMessage } from 'element-plus'
import * as G6 from '@antv/g6'

// 定义 props
const props = defineProps({
  astText: {
    type: String,
    required: true
  }
})

// 响应式数据
const astTree = ref(null)
const astInfo = ref({
  functions: [],
  variables: [],
  literals: [],
  structures: []
})
const stats = ref({
  nodeCount: 0,
  indentLevels: 0,
  nonEmptyLines: 0,
  totalLines: 0,
  estimatedSize: 0
})
const selectedNode = ref(null)
const showLabels = ref(true)
const showRawAst = ref(false)
const panelExpanded = ref(true)
const isLoading = ref(false)
const layout = ref('compactBox')

// 图实例
let graph = null
const graphContainer = ref(null)

// 初始化 AST 解析
const initAst = () => {
  if (!props.astText || !props.astText.trim()) {
    ElMessage.warning('AST 文本为空')
    return
  }

  isLoading.value = true

  try {
    // 解析 AST
    astTree.value = parseAstToTree(props.astText)

    // 提取信息
    astInfo.value = extractAstInfo(props.astText)

    // 获取统计信息
    stats.value = getAstStats(props.astText)

    // 更新图表
    updateGraph()

    ElMessage.success('AST 解析完成')
  } catch (error) {
    console.error('AST 解析失败:', error)
    ElMessage.error('AST 解析失败: ' + error.message)
  } finally {
    isLoading.value = false
  }
}

// 更新图表
const updateGraph = () => {
  if (!graph || !graphContainer.value || !astTree.value) return

  // 转换为图数据
  const graphData = convertToGraphData(astTree.value)

  // 配置布局
  const layoutConfig = getLayoutConfig(layout.value)

  // 更新图表数据
  graph.data({
    nodes: graphData.nodes,
    edges: graphData.edges
  })

  // 应用布局
  graph.updateLayout(layoutConfig)

  // 渲染
  graph.render()

  // 适应视图
  graph.fitView()
}

// 获取布局配置
const getLayoutConfig = (type) => {
  const baseConfig = {
    type,
    direction: 'LR', // 从左到右
    getHeight: () => {
      return 16
    },
    getWidth: () => {
      return 16
    },
    getVGap: () => {
      return 20
    },
    getHGap: () => {
      return 60
    }
  }

  switch (type) {
    case 'dendrogram':
      return {
        ...baseConfig,
        type: 'dendrogram',
        direction: 'LR',
        nodeSep: 30,
        rankSep: 100
      }
    case 'mindmap':
      return {
        ...baseConfig,
        type: 'mindmap',
        direction: 'H',
        getHeight: () => {
          return 16
        },
        getWidth: () => {
          return 16
        },
        getVGap: () => {
          return 10
        },
        getHGap: () => {
          return 50
        }
      }
    default: // compactBox
      return {
        ...baseConfig,
        type: 'compactBox',
        direction: 'LR',
        getHeight: () => {
          return 16
        },
        getWidth: () => {
          return 16
        },
        getVGap: () => {
          return 10
        },
        getHGap: () => {
          return 80
        },
        radial: true
      }
  }
}

// 初始化图表
const initGraph = () => {
  if (!graphContainer.value) return

  // 销毁现有图表
  if (graph) {
    graph.destroy()
  }

  // 创建图表
  graph = new G6.TreeGraph({
    container: graphContainer.value,
    width: graphContainer.value.clientWidth,
    height: graphContainer.value.clientHeight,
    modes: {
      default: ['drag-canvas', 'zoom-canvas', 'drag-node']
    },
    defaultNode: {
      type: 'circle',
      size: 20,
      style: {
        fill: '#1890ff',
        stroke: '#1890ff'
      },
      labelCfg: {
        style: {
          fontSize: 12,
          fill: '#fff'
        }
      }
    },
    defaultEdge: {
      type: 'line',
      style: {
        stroke: '#8c8c8c',
        lineWidth: 1,
        opacity: 0.6
      }
    },
    layout: getLayoutConfig(layout.value)
  })

  // 绑定事件
  graph.on('node:click', (evt) => {
    const node = evt.item
    const model = node.getModel()
    selectedNode.value = model

    // 高亮当前节点和路径
    highlightNodePath(node)
  })

  graph.on('canvas:click', () => {
    selectedNode.value = null
    // 清除高亮
    graph.getNodes().forEach(node => {
      graph.clearItemStates(node)
    })
  })

  // 监听窗口大小变化
  window.addEventListener('resize', handleResize)
}

// 高亮节点路径
const highlightNodePath = (node) => {
  // 清除之前的高亮
  graph.getNodes().forEach(n => {
    graph.clearItemStates(n)
  })
  graph.getEdges().forEach(e => {
    graph.clearItemStates(e)
  })

  // 高亮当前节点
  graph.setItemState(node, 'selected', true)

  // 高亮祖先路径
  let parent = node
  while (parent) {
    graph.setItemState(parent, 'highlight', true)
    const edges = parent.getInEdges()
    if (edges && edges.length > 0) {
      edges.forEach(edge => {
        graph.setItemState(edge, 'highlight', true)
      })
    }
    parent = parent.getParent()
  }
}

// 处理工具栏命令
const handleCommand = (command) => {
  switch (command) {
    case 'toggleLabels':
      showLabels.value = !showLabels.value
      graph.getNodes().forEach(node => {
        graph.updateItem(node, {
          labelCfg: {
            style: {
              fontSize: 12,
              fill: showLabels.value ? '#fff' : 'transparent'
            }
          }
        })
      })
      break
    case 'zoomIn':
      graph.zoom(1.1)
      break
    case 'zoomOut':
      graph.zoom(0.9)
      break
    case 'fitView':
      graph.fitView()
      break
    case 'downloadImage':
      downloadImage()
      break
  }
}

// 更改布局
const changeLayout = (newLayout) => {
  layout.value = newLayout
  updateGraph()
}

// 切换面板
const togglePanel = () => {
  panelExpanded.value = !panelExpanded.value
}

// 导出 JSON
const exportJson = () => {
  if (!astTree.value) return

  const dataStr = JSON.stringify(astTree.value, null, 2)
  const dataUri = 'data:application/json;charset=utf-8,' + encodeURIComponent(dataStr)

  const exportFileDefaultName = `ast-tree-${Date.now()}.json`
  const linkElement = document.createElement('a')
  linkElement.setAttribute('href', dataUri)
  linkElement.setAttribute('download', exportFileDefaultName)
  linkElement.click()

  ElMessage.success('AST JSON 导出成功')
}

// 复制 AST
const copyAst = async () => {
  try {
    await navigator.clipboard.writeText(props.astText)
    ElMessage.success('AST 文本已复制到剪贴板')
  } catch (err) {
    console.error('复制失败:', err)
    ElMessage.error('复制失败: ' + err.message)
  }
}

// 下载图片
const downloadImage = () => {
  if (!graph) return

  graph.downloadFullImage('AST-树状图-' + Date.now(), 'image/png', {
    backgroundColor: '#0f172a',
    padding: 20
  })

  ElMessage.success('图片保存成功')
}

// 处理窗口大小变化
const handleResize = () => {
  if (graph && graphContainer.value) {
    graph.changeSize(
        graphContainer.value.clientWidth,
        graphContainer.value.clientHeight
    )
    graph.fitView()
  }
}

// 监听 AST 文本变化
watch(() => props.astText, () => {
  initAst()
})

// 组件挂载时初始化
onMounted(() => {
  nextTick(() => {
    initGraph()
    initAst()
  })
})

// 组件卸载时清理
onUnmounted(() => {
  if (graph) {
    graph.destroy()
  }
  window.removeEventListener('resize', handleResize)
})
</script>

<style scoped>
.ast-tree-view {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: rgba(15, 23, 42, 0.8);
  border-radius: 12px;
  border: 1px solid #334155;
  overflow: hidden;
}

.ast-toolbar {
  padding: 15px 20px;
  background: rgba(15, 23, 42, 0.9);
  border-bottom: 1px solid #334155;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.toolbar-left h3 {
  margin: 0 0 10px 0;
  color: #f1f5f9;
  font-size: 1.3rem;
  display: flex;
  align-items: center;
  gap: 10px;
}

.toolbar-left h3 i {
  color: #60a5fa;
}

.stats {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
}

.stats .el-tag {
  background: rgba(99, 102, 241, 0.1);
  border: 1px solid rgba(99, 102, 241, 0.3);
  color: #818cf8;
}

.toolbar-right {
  display: flex;
  align-items: center;
  gap: 10px;
}

.layout-dropdown {
  margin-left: 10px;
}

.ast-main {
  flex: 1;
  display: flex;
  overflow: hidden;
  min-height: 500px;
}

.tree-container {
  flex: 1;
  padding: 20px;
  display: flex;
  flex-direction: column;
  border-right: 1px solid #334155;
}

.graph-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 15px;
}

.graph-header h4 {
  margin: 0;
  color: #e2e8f0;
  font-size: 1.1rem;
}

.graph-wrapper {
  flex: 1;
  border-radius: 8px;
  border: 1px solid #334155;
  background: #0d1117;
  overflow: hidden;
  position: relative;
  min-height: 400px;
}

.graph-legend {
  display: flex;
  gap: 20px;
  margin-top: 15px;
  padding-top: 15px;
  border-top: 1px solid #334155;
  flex-wrap: wrap;
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 8px;
  color: #94a3b8;
  font-size: 0.9rem;
}

.legend-color {
  width: 12px;
  height: 12px;
  border-radius: 3px;
}

.info-panel {
  width: 350px;
  background: rgba(15, 23, 42, 0.9);
  display: flex;
  flex-direction: column;
  transition: width 0.3s ease;
}

.info-panel.collapsed {
  width: 40px;
}

.panel-header {
  padding: 15px 20px;
  border-bottom: 1px solid #334155;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.panel-header h4 {
  margin: 0;
  color: #e2e8f0;
  font-size: 1.1rem;
}

.panel-content {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.node-details {
  background: rgba(30, 41, 59, 0.5);
  border-radius: 8px;
  padding: 15px;
  margin-bottom: 20px;
  border: 1px solid #334155;
}

.node-details h5 {
  margin: 0 0 15px 0;
  color: #f1f5f9;
  font-size: 1rem;
  display: flex;
  align-items: center;
  gap: 8px;
}

.detail-item {
  display: flex;
  margin-bottom: 10px;
  align-items: flex-start;
}

.detail-item.full {
  flex-direction: column;
}

.detail-label {
  color: #94a3b8;
  font-size: 0.9rem;
  min-width: 80px;
  flex-shrink: 0;
}

.detail-value {
  color: #e2e8f0;
  font-size: 0.9rem;
  word-break: break-all;
  flex: 1;
}

.detail-value pre {
  margin: 5px 0 0 0;
  font-size: 0.85rem;
  line-height: 1.4;
  white-space: pre-wrap;
  background: rgba(0, 0, 0, 0.3);
  padding: 10px;
  border-radius: 4px;
  max-height: 200px;
  overflow-y: auto;
}

.stats-section {
  margin-top: 20px;
}

.stats-section h5 {
  margin: 0 0 15px 0;
  color: #f1f5f9;
  font-size: 1rem;
  display: flex;
  align-items: center;
  gap: 8px;
}

.info-section {
  margin-bottom: 20px;
}

.info-section h6 {
  margin: 0 0 10px 0;
  color: #cbd5e1;
  font-size: 0.9rem;
  display: flex;
  align-items: center;
  gap: 6px;
}

.info-items {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.info-tag {
  background: rgba(99, 102, 241, 0.1);
  border: 1px solid rgba(99, 102, 241, 0.3);
  color: #818cf8;
}

.action-buttons {
  margin-top: 20px;
  display: flex;
  gap: 10px;
  justify-content: center;
}

.raw-ast-section {
  border-top: 1px solid #334155;
  background: rgba(15, 23, 42, 0.9);
}

.section-header {
  padding: 15px 20px;
  border-bottom: 1px solid #334155;
  display: flex;
  justify-content: space-between;
  align-items: center;
  cursor: pointer;
  transition: background 0.3s ease;
}

.section-header:hover {
  background: rgba(30, 41, 59, 0.5);
}

.section-header h4 {
  margin: 0;
  color: #e2e8f0;
  font-size: 1.1rem;
  display: flex;
  align-items: center;
  gap: 10px;
}

.raw-ast-content {
  padding: 20px;
  max-height: 300px;
  overflow-y: auto;
}

.ast-text {
  margin: 0;
  font-family: 'JetBrains Mono', 'Fira Code', monospace;
  font-size: 12px;
  line-height: 1.5;
  color: #c9d1d9;
  white-space: pre-wrap;
  word-break: break-all;
  background: #0d1117;
  padding: 15px;
  border-radius: 8px;
  border: 1px solid #334155;
  max-height: 250px;
  overflow-y: auto;
}

/* 响应式设计 */
@media (max-width: 1200px) {
  .ast-main {
    flex-direction: column;
  }

  .tree-container {
    border-right: none;
    border-bottom: 1px solid #334155;
    min-height: 400px;
  }

  .info-panel {
    width: 100%;
    height: 300px;
  }
}

@media (max-width: 768px) {
  .ast-toolbar {
    flex-direction: column;
    align-items: flex-start;
    gap: 15px;
  }

  .toolbar-right {
    width: 100%;
    justify-content: space-between;
  }

  .graph-legend {
    flex-direction: column;
    gap: 10px;
  }
}
</style>