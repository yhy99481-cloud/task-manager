import React from 'react';
import { useTaskStore } from '../../store/useStore';
import { Button } from '../ui/Button';
import type { Task } from '../../types';

interface TaskItemProps {
  task: Task;
  onEdit: (task: Task) => void;
}

const statusConfig: Record<Task['status'], { color: string; label: string; gradient: string }> = {
  todo: {
    color: 'text-gray-700',
    label: 'To Do',
    gradient: 'from-gray-100 to-gray-200',
  },
  in_progress: {
    color: 'text-blue-700',
    label: 'In Progress',
    gradient: 'from-blue-100 to-blue-200',
  },
  done: {
    color: 'text-green-700',
    label: 'Done',
    gradient: 'from-green-100 to-green-200',
  },
};

export const TaskItem: React.FC<TaskItemProps> = ({ task, onEdit }) => {
  const { deleteTask, updateTask } = useTaskStore();
  const config = statusConfig[task.status];

  const handleStatusChange = async (status: Task['status']) => {
    try {
      await updateTask(task.id, { status });
    } catch (error) {
      console.error('Failed to update status:', error);
    }
  };

  const handleDelete = async () => {
    if (window.confirm('Are you sure you want to delete this task?')) {
      try {
        await deleteTask(task.id);
      } catch (error) {
        console.error('Failed to delete task:', error);
      }
    }
  };

  const nextStatus: Record<Task['status'], Task['status']> = {
    todo: 'in_progress',
    in_progress: 'done',
    done: 'todo',
  };

  return (
    <div className="group card p-6 hover:scale-[1.02] transition-all duration-300 animate-fade-in">
      {/* Status Badge */}
      <div className="flex items-start justify-between mb-4">
        <div className="flex-1">
          <h3 className="text-lg font-bold text-gray-800 mb-2">{task.title}</h3>
          <p className="text-gray-600 text-sm leading-relaxed">{task.description}</p>
        </div>
        <span className={`flex-shrink-0 ml-4 px-3 py-1.5 rounded-full text-xs font-bold bg-gradient-to-r ${config.gradient} ${config.color} shadow-sm`}>
          {config.label}
        </span>
      </div>

      {/* Footer */}
      <div className="flex items-center justify-between pt-4 border-t border-gray-100">
        <div className="flex items-center gap-1.5 text-sm text-gray-400">
          <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
          </svg>
          {new Date(task.created_at).toLocaleDateString()}
        </div>

        {/* Action Buttons */}
        <div className="flex items-center gap-2 opacity-0 group-hover:opacity-100 transition-opacity duration-200">
          <Button
            size="sm"
            variant="secondary"
            onClick={() => handleStatusChange(nextStatus[task.status])}
          >
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M13 7l5 5m0 0l-5 5m5-5H6" />
            </svg>
            {statusConfig[nextStatus[task.status]].label}
          </Button>
          <Button size="sm" variant="ghost" onClick={() => onEdit(task)}>
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
            </svg>
          </Button>
          <Button size="sm" variant="danger" onClick={handleDelete}>
            <svg className="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
            </svg>
          </Button>
        </div>
      </div>
    </div>
  );
};
