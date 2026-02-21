import api from './client';
import type {
  CreateTaskRequest,
  Task,
  TasksResponse,
  UpdateTaskRequest,
  TaskFilter,
} from '../types';

export const tasksApi = {
  getTasks: (filter?: TaskFilter) =>
    api.get<TasksResponse>('/api/tasks', { params: filter }),

  getTask: (id: string) =>
    api.get<Task>(`/api/tasks/${id}`),

  createTask: (data: CreateTaskRequest) =>
    api.post<Task>('/api/tasks', data),

  updateTask: (id: string, data: UpdateTaskRequest) =>
    api.put<Task>(`/api/tasks/${id}`, data),

  updateTaskStatus: (id: string, status: string) =>
    api.patch<Task>(`/api/tasks/${id}/status`, { status }),

  deleteTask: (id: string) =>
    api.delete(`/api/tasks/${id}`),
};
