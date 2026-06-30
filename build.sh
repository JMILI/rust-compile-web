#!/bin/bash
# Vercel 构建脚本：仅构建 Vue 前端（静态站点）
# 编译 API 需单独部署 backend/（见 DEPLOY.md）
set -e

cd "$(dirname "$0")/frontend"

echo "==> npm install"
npm install

echo "==> vite build"
npm run build

echo "==> done: frontend/dist"
