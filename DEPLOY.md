# rust-compile-web 部署指南

本项目是 **前后端分离** 架构，与 `cPlusBook`（纯静态 mdBook）不同：

| 部分 | 技术 | 能否放 Vercel |
|------|------|----------------|
| **frontend/** | Vue 3 + Vite | ✅ 可以（静态站点） |
| **backend/** | Rust Actix + **nightly rustc** | ❌ 不可以（需完整 Rust 工具链） |

推荐：**Vercel 托管前端 + Railway/Fly.io/VPS 托管后端**。

---

## 一、Vercel 部署前端（与 cPlusBook 类似）

### 仓库内已添加的文件

| 文件 | 作用 |
|------|------|
| `vercel.json` | 构建命令、输出目录、SPA 路由 |
| `build.sh` | `npm install` + `vite build` |
| `.vercelignore` | 忽略 backend，加快上传 |
| `frontend/.env.example` | 生产环境 API 地址说明 |

### Vercel 控制台设置

1. **Import Git Repository** → 选择本仓库
2. **Root Directory**：`deployProject/rust-compile-web`（或你推送的子路径）
3. Framework Preset：**Other**（已用 `vercel.json` 指定构建）
4. **Environment Variables**（Production）：

   | 变量名 | 值示例 |
   |--------|--------|
   | `VITE_API_BASE_URL` | `https://你的后端域名/api` |

5. Deploy

构建成功后访问 Vercel 分配的域名即为前端界面。

### 本地验证构建（与 Vercel 一致）

```bash
chmod +x build.sh
./build.sh
# 静态文件在 frontend/dist/
```

---

## 二、部署后端 API

后端需要 **Rust nightly**（`-Z unpretty=...` 等不稳定特性）。

### 方式 A：Docker（推荐）

```bash
cd backend
docker build -t rust-compile-api .
docker run -p 8080:8080 -e PORT=8080 rust-compile-api
```

在 Railway / Fly.io 中：**New Project → Deploy from Dockerfile**，根目录选 `backend/`。

部署完成后得到公网 URL，例如 `https://rust-compile-api.up.railway.app`。

在 Vercel 设置：

```
VITE_API_BASE_URL=https://rust-compile-api.up.railway.app/api
```

然后 **Redeploy** 前端使环境变量生效。

### 方式 B：裸机 / VPS

```bash
cd backend
rustup override set nightly
cargo build --release
HOST=0.0.0.0 PORT=8080 ./target/release/rust-compile-backend
```

---

## 三、本地开发

```bash
# 终端 1：后端
cd backend && rustup override set nightly && cargo run

# 终端 2：前端
cd frontend && npm install && npm run dev
```

前端：http://127.0.0.1:5173  
后端：http://127.0.0.1:8080  

本地无需设置 `VITE_API_BASE_URL`（开发模式直连 8080）。

---

## 四、与 cPlusBook 对比

| | cPlusBook | rust-compile-web |
|--|-----------|------------------|
| Vercel 产物 | `book/` 静态 HTML | `frontend/dist/` 静态 SPA |
| 构建脚本 | `build.sh` → mdbook | `build.sh` → vite build |
| 运行时依赖 | 无 | 后端需 nightly rustc |
| 环境变量 | 无 | `VITE_API_BASE_URL` |

---

## 五、常见问题

**Q: Vercel 部署后点击「开始探索」失败？**  
A: 未配置后端或未设置 `VITE_API_BASE_URL`。先部署 backend，再在 Vercel 填 API 地址并重新部署。

**Q: CORS 错误？**  
A: 后端已配置 `allow_any_origin()`，确认 `VITE_API_BASE_URL` 以 `/api` 结尾且与后端路由一致。

**Q: 能否把整个项目都放 Vercel？**  
A: 不能。Vercel Serverless 无法安装 nightly rustc，也无法在函数里长时间跑 `rustc` 子进程。
