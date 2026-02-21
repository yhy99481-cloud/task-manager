# Task Manager - Quick Start Guide

## 项目已完成！

这个任务管理系统包含以下功能：

### 核心功能
- 用户注册/登录（JWT认证）
- 任务的增删改查
- 任务状态管理（Todo、In Progress、Done）
- 任务搜索（标题模糊匹配）
- 按状态筛选
- 分页显示
- 列表视图和看板视图
- 拖拽改变任务状态

### 技术栈
- **后端**: Rust + Axum + SQLite + JWT
- **前端**: React + TypeScript + Tailwind CSS + Zustand
- **部署**: Docker + Docker Compose

---

## 启动方式

### 方式一：本地开发

#### 启动后端
```bash
cd D:\projects\task-manager\backend
cargo run
```
后端运行在 http://localhost:3000

#### 启动前端（新终端）
```bash
cd D:\projects\task-manager\frontend
npm run dev
```
前端运行在 http://localhost:5173

### 方式二：Docker 部署
```bash
cd D:\projects\task-manager
docker-compose up --build
```
访问 http://localhost

---

## 使用说明

1. 打开浏览器访问前端地址
2. 注册一个新账户
3. 登录后可以：
   - 创建任务
   - 编辑/删除任务
   - 修改任务状态
   - 搜索任务
   - 按状态筛选
   - 切换列表/看板视图
   - 在看板视图中拖拽任务

---

## API 端点

### 认证
- `POST /api/register` - 用户注册
- `POST /api/login` - 用户登录

### 任务
- `GET /api/tasks` - 获取任务列表（支持分页、搜索、筛选）
- `POST /api/tasks` - 创建任务
- `GET /api/tasks/:id` - 获取单个任务
- `PUT /api/tasks/:id` - 更新任务
- `PATCH /api/tasks/:id/status` - 更新任务状态
- `DELETE /api/tasks/:id` - 删除任务

### 查询参数
- `page` - 页码（默认：1）
- `limit` - 每页数量（默认：10，最大：100）
- `status` - 状态筛选：`todo`、`in_progress`、`done`
- `search` - 搜索关键词（标题模糊匹配）
