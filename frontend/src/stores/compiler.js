import { defineStore } from 'pinia'
import compilerApi from '@/api/compiler'

export const useCompilerStore = defineStore('compiler', {
    state: () => ({
        code: '',
        currentStage: 'ast',
        compileOptions: {
            verbose: false,
            brief: false,
            fullOutput: false
        },
        compileResult: null,
        isLoading: false,
        error: null,
        availableStages: [],
        rustVersion: null
    }),

    actions: {
        // 设置代码
        setCode(newCode) {
            this.code = newCode
        },

        // 设置编译阶段
        setStage(stage) {
            this.currentStage = stage
        },

        // 更新编译选项
        updateOptions(options) {
            this.compileOptions = { ...this.compileOptions, ...options }
        },

        // 获取可用阶段
        async fetchStages() {
            try {
                this.availableStages = await compilerApi.getStages()
            } catch (error) {
                console.error('Failed to fetch stages:', error)
                this.availableStages = []
            }
        },

        // 获取 Rust 版本
        async fetchRustVersion() {
            try {
                this.rustVersion = await compilerApi.getRustVersion()
            } catch (error) {
                console.error('Failed to fetch Rust version:', error)
                this.rustVersion = { version: 'Unknown', is_nightly: false }
            }
        },

        // 编译代码
        async compile() {
            if (!this.code.trim()) {
                this.error = 'Please enter some Rust code'
                return
            }

            this.isLoading = true
            this.error = null

            try {
                const result = await compilerApi.compileCode(
                    this.code,
                    this.currentStage,
                    this.compileOptions
                )

                this.compileResult = result

                if (!result.success) {
                    this.error = result.error || 'Compilation failed'
                }
            } catch (error) {
                this.error = error.response?.data?.error || error.message || 'Network error'
                this.compileResult = null
            } finally {
                this.isLoading = false
            }
        },

        // 重置状态
        reset() {
            this.code = ''
            this.currentStage = 'ast'
            this.compileOptions = {
                verbose: false,
                brief: false,
                fullOutput: false
            }
            this.compileResult = null
            this.error = null
            this.isLoading = false
        }
    },

    getters: {
        // 获取当前阶段信息
        currentStageInfo: (state) => {
            return state.availableStages.find(stage => stage.id === state.currentStage)
        },

        // 检查是否有编译结果
        hasResult: (state) => {
            return state.compileResult !== null && state.compileResult.success
        }
    }
})