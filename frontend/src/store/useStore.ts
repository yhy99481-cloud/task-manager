import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { apiClient } from '../api/client';
import { tasksApi } from '../api/tasks';
import type {
  User,
  Task,
  TaskFilter,
  LoginRequest,
  RegisterRequest,
  CreateTaskRequest,
  UpdateTaskRequest,
} from '../types';

interface AuthState {
  user: User | null;
  token: string | null;
  isAuthenticated: boolean;
  login: (credentials: LoginRequest) => Promise<void>;
  register: (data: RegisterRequest) => Promise<void>;
  logout: () => void;
  loadUser: () => void;
}

interface TaskState {
  tasks: Task[];
  filter: TaskFilter;
  pagination: {
    page: number;
    limit: number;
    total: number;
    totalPages: number;
  };
  loading: boolean;
  error: string | null;
  fetchTasks: () => Promise<void>;
  setFilter: (filter: Partial<TaskFilter>) => void;
  createTask: (data: CreateTaskRequest) => Promise<void>;
  updateTask: (id: string, data: UpdateTaskRequest) => Promise<void>;
  deleteTask: (id: string) => Promise<void>;
  clearError: () => void;
}

// Auth Store
export const useAuthStore = create<AuthState>()(
  persist(
    (set) => ({
      user: null,
      token: null,
      isAuthenticated: false,

      login: async (credentials: LoginRequest) => {
        const response = await apiClient.login(credentials.username, credentials.password);
        const { token, user } = response.data;
        localStorage.setItem('token', token);
        localStorage.setItem('user', JSON.stringify(user));
        set({ token, user, isAuthenticated: true });
      },

      register: async (data: RegisterRequest) => {
        const response = await apiClient.register(data.username, data.password);
        const { token, user } = response.data;
        localStorage.setItem('token', token);
        localStorage.setItem('user', JSON.stringify(user));
        set({ token, user, isAuthenticated: true });
      },

      logout: () => {
        apiClient.logout();
        set({ user: null, token: null, isAuthenticated: false });
      },

      loadUser: () => {
        const token = localStorage.getItem('token');
        const userStr = localStorage.getItem('user');
        const user = userStr ? JSON.parse(userStr) : null;
        set({ token, user, isAuthenticated: !!token });
      },
    }),
    {
      name: 'auth-storage',
      partialize: (state) => ({ token: state.token, user: state.user }),
    }
  )
);

// Task Store
export const useTaskStore = create<TaskState>()((set, get) => ({
  tasks: [],
  filter: { page: 1, limit: 10 },
  pagination: { page: 1, limit: 10, total: 0, totalPages: 0 },
  loading: false,
  error: null,

  fetchTasks: async () => {
    set({ loading: true, error: null });
    try {
      const { filter } = get();
      const response = await tasksApi.getTasks(filter);
      // Map total_pages to totalPages
      const pagination = {
        ...response.data.pagination,
        totalPages: (response.data.pagination as any).total_pages,
      };
      set({
        tasks: response.data.tasks,
        pagination,
        loading: false,
      });
    } catch (error: any) {
      set({
        error: error.response?.data?.message || 'Failed to fetch tasks',
        loading: false,
      });
    }
  },

  setFilter: (filter: Partial<TaskFilter>) => {
    const currentFilter = get().filter;
    set({
      filter: { ...currentFilter, ...filter, page: filter.page ?? 1 },
    });
  },

  createTask: async (data: CreateTaskRequest) => {
    set({ loading: true, error: null });
    try {
      await tasksApi.createTask(data);
      await get().fetchTasks();
    } catch (error: any) {
      set({
        error: error.response?.data?.message || 'Failed to create task',
        loading: false,
      });
      throw error;
    }
  },

  updateTask: async (id: string, data: UpdateTaskRequest) => {
    set({ loading: true, error: null });
    try {
      await tasksApi.updateTask(id, data);
      await get().fetchTasks();
    } catch (error: any) {
      set({
        error: error.response?.data?.message || 'Failed to update task',
        loading: false,
      });
      throw error;
    }
  },

  deleteTask: async (id: string) => {
    set({ loading: true, error: null });
    try {
      await tasksApi.deleteTask(id);
      await get().fetchTasks();
    } catch (error: any) {
      set({
        error: error.response?.data?.message || 'Failed to delete task',
        loading: false,
      });
      throw error;
    }
  },

  clearError: () => set({ error: null }),
}));
