import React, { useState, useEffect } from 'react';
import {
  DndContext,
  DragOverlay,
  PointerSensor,
  useSensor,
  useSensors,
  closestCenter,
  type DragEndEvent,
  type DragStartEvent,
} from '@dnd-kit/core';
import { useTaskStore } from '../../store/useStore';
import { TaskForm } from './TaskForm';
import { Modal } from '../ui/Modal';
import { Button } from '../ui/Button';
import { Loading } from '../ui/Loading';
import type { Task, TaskStatus } from '../../types';

interface KanbanColumnProps {
  status: TaskStatus;
  title: string;
  icon: string;
  gradient: string;
  tasks: Task[];
  onEdit: (task: Task) => void;
}

const KanbanColumn: React.FC<KanbanColumnProps> = ({ title, icon, gradient, tasks, onEdit }) => {
  return (
    <div className="flex flex-col h-full">
      <div className={`mb-4 flex items-center gap-2 px-4 py-2 rounded-xl bg-gradient-to-r ${gradient} text-white`}>
        <span className="text-lg">{icon}</span>
        <h2 className="font-semibold">{title}</h2>
        <span className="ml-auto px-2 py-0.5 rounded-full bg-white/20 text-sm font-bold">
          {tasks.length}
        </span>
      </div>
      <div className="flex-1 space-y-3 min-h-[200px]">
        {tasks.map((task) => (
          <div
            key={task.id}
            className="group cursor-grab p-4 rounded-xl bg-white shadow-sm hover:shadow-lg transition-all duration-200 active:cursor-grabbing border-2 border-transparent hover:border-indigo-200"
            onClick={() => onEdit(task)}
          >
            <h3 className="font-bold text-gray-800 mb-2">{task.title}</h3>
            <p className="text-sm text-gray-600 line-clamp-3">{task.description}</p>
            <div className="mt-3 flex items-center gap-1 text-xs text-gray-400">
              <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
              </svg>
              {new Date(task.created_at).toLocaleDateString()}
            </div>
          </div>
        ))}
        {tasks.length === 0 && (
          <div className="flex items-center justify-center h-32 rounded-xl border-2 border-dashed border-gray-200 text-gray-400">
            <p className="text-sm">Drop tasks here</p>
          </div>
        )}
      </div>
    </div>
  );
};

export const KanbanBoard: React.FC = () => {
  const { tasks, loading, fetchTasks, updateTask } = useTaskStore();
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [editingTask, setEditingTask] = useState<Task | undefined>();
  const [activeTask, setActiveTask] = useState<Task | null>(null);

  const sensors = useSensors(
    useSensor(PointerSensor, {
      activationConstraint: {
        distance: 8,
      },
    })
  );

  useEffect(() => {
    fetchTasks();
  }, [fetchTasks]);

  const handleDragStart = (event: DragStartEvent) => {
    const { active } = event;
    const task = tasks.find((t) => t.id === active.id);
    if (task) {
      setActiveTask(task);
    }
  };

  const handleDragEnd = async (event: DragEndEvent) => {
    const { active, over } = event;
    setActiveTask(null);

    if (over && active.id !== over.id) {
      const task = tasks.find((t) => t.id === active.id);
      if (task) {
        const newStatus = over.id as TaskStatus;
        if (task.status !== newStatus) {
          try {
            await updateTask(task.id, { status: newStatus });
          } catch (error) {
            console.error('Failed to update task status:', error);
          }
        }
      }
    }
  };

  const handleCreate = () => {
    setEditingTask(undefined);
    setIsModalOpen(true);
  };

  const handleEdit = (task: Task) => {
    setEditingTask(task);
    setIsModalOpen(true);
  };

  const closeModal = () => {
    setIsModalOpen(false);
    setEditingTask(undefined);
  };

  const tasksByStatus: Record<TaskStatus, Task[]> = {
    todo: tasks.filter((t) => t.status === 'todo'),
    in_progress: tasks.filter((t) => t.status === 'in_progress'),
    done: tasks.filter((t) => t.status === 'done'),
  };

  if (loading && tasks.length === 0) {
    return <Loading message="Loading tasks..." />;
  }

  return (
    <div className="container mx-auto px-4 py-8 max-w-7xl">
      {/* Header */}
      <div className="mb-8 flex flex-col sm:flex-row sm:items-center sm:justify-between gap-4">
        <div>
          <h1 className="text-4xl font-bold bg-gradient-to-r from-blue-600 to-indigo-600 bg-clip-text text-transparent mb-2">
            Kanban Board
          </h1>
          <p className="text-gray-500">Drag and drop tasks to update status</p>
        </div>
        <Button onClick={handleCreate} size="lg">
          <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
          </svg>
          New Task
        </Button>
      </div>

      <DndContext
        sensors={sensors}
        collisionDetection={closestCenter}
        onDragStart={handleDragStart}
        onDragEnd={handleDragEnd}
      >
        <div className="grid gap-6 md:grid-cols-3">
          <div data-id="todo" id="todo" className="glass-effect rounded-2xl p-6">
            <KanbanColumn
              status="todo"
              title="To Do"
              icon="📋"
              gradient="from-gray-500 to-gray-600"
              tasks={tasksByStatus.todo}
              onEdit={handleEdit}
            />
          </div>
          <div data-id="in_progress" id="in_progress" className="glass-effect rounded-2xl p-6">
            <KanbanColumn
              status="in_progress"
              title="In Progress"
              icon="🚀"
              gradient="from-blue-500 to-indigo-500"
              tasks={tasksByStatus.in_progress}
              onEdit={handleEdit}
            />
          </div>
          <div data-id="done" id="done" className="glass-effect rounded-2xl p-6">
            <KanbanColumn
              status="done"
              title="Done"
              icon="✅"
              gradient="from-green-500 to-emerald-500"
              tasks={tasksByStatus.done}
              onEdit={handleEdit}
            />
          </div>
        </div>

        <DragOverlay>
          {activeTask ? (
            <div className="rotate-3 w-80 p-4 rounded-xl bg-white shadow-2xl border-2 border-indigo-300">
              <h3 className="font-bold text-gray-800 mb-2">{activeTask.title}</h3>
              <p className="text-sm text-gray-600">{activeTask.description}</p>
            </div>
          ) : null}
        </DragOverlay>
      </DndContext>

      <Modal
        isOpen={isModalOpen}
        onClose={closeModal}
        title={editingTask ? 'Edit Task' : 'Create Task'}
      >
        <TaskForm onClose={closeModal} task={editingTask} />
      </Modal>
    </div>
  );
};
