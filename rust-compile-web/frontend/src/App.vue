<template>
  <div id="app">
    <router-view />
  </div>
</template>

<script setup>
import { onMounted } from 'vue'

onMounted(() => {
  // 页面加载完成后的初始化
  console.log('Rust Compile Explorer loaded')
})
</script>

<style>
/* 全局样式重置和优化 */
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen',
  'Ubuntu', 'Cantarell', 'Fira Sans', 'Droid Sans', 'Helvetica Neue',
  sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  background: #0f172a;
  color: #fff;
  line-height: 1.5;
}

#app {
  min-height: 100vh;
  background: linear-gradient(135deg, #0f172a 0%, #1a202c 100%);
}

/* 自定义滚动条 */
::-webkit-scrollbar {
  width: 10px;
  height: 10px;
}

::-webkit-scrollbar-track {
  background: rgba(15, 23, 42, 0.6);
  border-radius: 5px;
}

::-webkit-scrollbar-thumb {
  background: linear-gradient(135deg, #3b82f6, #1d4ed8);
  border-radius: 5px;
}

::-webkit-scrollbar-thumb:hover {
  background: linear-gradient(135deg, #60a5fa, #3b82f6);
}

/* Element Plus 组件样式覆盖 */
.el-button {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  font-weight: 500;
  border-radius: 8px;
}

.el-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.el-button--primary {
  background: linear-gradient(90deg, #3b82f6, #2563eb);
  border: none;
}

.el-button--primary:hover {
  background: linear-gradient(90deg, #60a5fa, #3b82f6);
  box-shadow: 0 4px 20px rgba(59, 130, 246, 0.4);
}

.el-tag {
  font-weight: 500;
  border-radius: 20px;
  padding: 4px 12px;
}

.el-card {
  background: rgba(15, 23, 42, 0.8);
  border: 1px solid #334155;
  border-radius: 12px;
  backdrop-filter: blur(10px);
}

.el-card:hover {
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
}

.el-alert {
  border-radius: 10px;
  backdrop-filter: blur(10px);
}

.el-alert--error {
  background: rgba(239, 68, 68, 0.1);
  border: 1px solid rgba(239, 68, 68, 0.2);
}

.el-alert--success {
  background: rgba(34, 197, 94, 0.1);
  border: 1px solid rgba(34, 197, 94, 0.2);
}

.el-alert--warning {
  background: rgba(245, 158, 11, 0.1);
  border: 1px solid rgba(245, 158, 11, 0.2);
}

.el-alert--info {
  background: rgba(148, 163, 184, 0.1);
  border: 1px solid rgba(148, 163, 184, 0.2);
}

.el-collapse {
  border: none;
}

.el-collapse-item__header {
  font-weight: 600;
  background: rgba(30, 41, 59, 0.8);
  border-bottom: 1px solid #334155;
  color: #e2e8f0;
}

.el-collapse-item__wrap {
  background: rgba(15, 23, 42, 0.9);
}

.el-checkbox__label {
  color: #cbd5e1;
}

.el-checkbox__inner {
  background-color: #1e293b;
  border-color: #475569;
}

.el-checkbox__inner:hover {
  border-color: #60a5fa;
}

/* 代码编辑器相关 */
.code-editor pre {
  font-family: 'JetBrains Mono', 'Fira Code', 'Cascadia Code', 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 14px;
  line-height: 1.6;
  tab-size: 2;
}

/* 动画效果 */
@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.fade-in {
  animation: fadeIn 0.5s ease-out;
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}

.pulse {
  animation: pulse 2s ease-in-out infinite;
}

/* 工具提示优化 */
.el-tooltip__popper {
  background: rgba(15, 23, 42, 0.95) !important;
  border: 1px solid #334155 !important;
  backdrop-filter: blur(10px);
  color: #fff !important;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
}

.el-tooltip__popper .popper__arrow {
  border-color: #334155 !important;
}

.el-tooltip__popper .popper__arrow::before {
  background: rgba(15, 23, 42, 0.95) !important;
  border-color: #334155 !important;
}

/* 加载状态 */
.loading-shimmer {
  background: linear-gradient(
      90deg,
      rgba(30, 41, 59, 0.4) 25%,
      rgba(30, 41, 59, 0.8) 50%,
      rgba(30, 41, 59, 0.4) 75%
  );
  background-size: 200% 100%;
  animation: shimmer 2s infinite;
}

@keyframes shimmer {
  0% {
    background-position: -200% 0;
  }
  100% {
    background-position: 200% 0;
  }
}


</style>