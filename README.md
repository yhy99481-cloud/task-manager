# Task Manager (Mini Project Manager)

A full-stack task management system with user authentication, task CRUD operations, search, filtering, pagination, and kanban board view with drag-and-drop.

## Features

### User Module
- User registration and login
- JWT-based authentication
- Each user can only see their own tasks

### Task Features
- Create, read, update, and delete tasks
- Task status management (Todo, In Progress, Done)
- Search tasks by title (fuzzy matching)
- Filter tasks by status
- Paginated task list
- Kanban board view with drag-and-drop

### Tech Stack

#### Backend
- **Rust** with **Axum** web framework
- **SQLite** for data persistence
- **JWT** for authentication
- **bcrypt** for password hashing

#### Frontend
- **React** with **TypeScript**
- **Vite** for build tooling
- **Tailwind CSS** for styling
- **Zustand** for state management
- **React Router** for navigation
- **@dnd-kit** for drag-and-drop functionality

#### Deployment
- **Docker** and **Docker Compose** for containerization

## Project Structure

```
task-manager/
├── backend/
│   ├── src/
│   │   ├── main.rs          # Application entry point
│   │   ├── models/          # Data models
│   │   ├── db/              # Database layer
│   │   ├── handlers/        # API handlers
│   │   ├── middleware/      # Authentication middleware
│   │   └── utils/           # Utilities (JWT)
│   ├── migrations/
│   ├── Cargo.toml
│   └── Dockerfile
├── frontend/
│   ├── src/
│   │   ├── api/             # API client
│   │   ├── store/           # Zustand stores
│   │   ├── components/      # React components
│   │   ├── types/           # TypeScript types
│   │   └── App.tsx
│   ├── Dockerfile
│   └── nginx.conf
├── docker-compose.yml
└── README.md
```

## Getting Started

### Prerequisites
- Docker and Docker Compose
- Node.js 20+ (for local development)
- Rust 1.75+ (for local development)

### Using Docker (Recommended)

1. Clone the repository:
```bash
git clone <repository-url>
cd task-manager
```

2. Create environment file:
```bash
cp backend/.env.example backend/.env
```

3. Start the application:
```bash
docker-compose up --build
```

4. Access the application:
- Frontend: http://localhost
- Backend API: http://localhost:3000

### Local Development

#### Backend

1. Navigate to the backend directory:
```bash
cd backend
```

2. Copy environment file:
```bash
cp .env.example .env
```

3. Run the application:
```bash
cargo run
```

The API will be available at http://localhost:3000

#### Frontend

1. Navigate to the frontend directory:
```bash
cd frontend
```

2. Install dependencies:
```bash
npm install
```

3. Start the development server:
```bash
npm run dev
```

The application will be available at http://localhost:5173

## API Endpoints

### Authentication
- `POST /api/register` - Register a new user
- `POST /api/login` - Login a user

### Tasks
- `GET /api/tasks` - Get all tasks (with pagination, search, and filter)
- `POST /api/tasks` - Create a new task
- `GET /api/tasks/:id` - Get a specific task
- `PUT /api/tasks/:id` - Update a task
- `PATCH /api/tasks/:id/status` - Update task status
- `DELETE /api/tasks/:id` - Delete a task

### Query Parameters
- `page` - Page number (default: 1)
- `limit` - Items per page (default: 10, max: 100)
- `status` - Filter by status: `todo`, `in_progress`, `done`
- `search` - Search in task title

## Usage

1. Register a new account or login
2. Create tasks with the "Create Task" button
3. View tasks in List View or Kanban Board
4. Edit or delete tasks
5. Filter tasks by status or search by title
6. Drag and drop tasks between columns in Kanban view

## Development

### Running Tests

Backend tests:
```bash
cd backend
cargo test
```

### Building for Production

Backend:
```bash
cd backend
cargo build --release
```

Frontend:
```bash
cd frontend
npm run build
```

## License

MIT License
