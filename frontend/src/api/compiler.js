import axios from 'axios'

// 生产环境在 Vercel 设置 VITE_API_BASE_URL；开发环境走 vite proxy /api
const API_BASE_URL =
    import.meta.env.VITE_API_BASE_URL ||
    (import.meta.env.DEV ? 'http://127.0.0.1:8080/api' : '/api')

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