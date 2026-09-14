import axios from 'axios'

// 开发/隧道访问走 Vite 同源代理 /api，避免浏览器请求 127.0.0.1
const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || '/api'

const apiClient = axios.create({
    baseURL: API_BASE_URL,
    headers: {
        'Content-Type': 'application/json',
    },
})

export default {
    // 编译代码
    async compileCode(code, stage, options = {}) {
        try {
            const response = await apiClient.post('/compile', {
                code,
                stage,
                options
            })
            return response.data
        } catch (error) {
            console.error('Compile error:', error)
            throw error
        }
    },

    // 获取可用的编译阶段
    async getStages() {
        try {
            const response = await apiClient.get('/stages')
            return response.data.stages
        } catch (error) {
            console.error('Get stages error:', error)
            return []
        }
    },

    // 获取 Rust 版本
    async getRustVersion() {
        try {
            const response = await apiClient.get('/version')
            return response.data
        } catch (error) {
            console.error('Get version error:', error)
            return { version: 'Unknown', is_nightly: false }
        }
    }
}