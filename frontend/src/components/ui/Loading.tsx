import React from 'react';

export const Loading: React.FC<{ message?: string }> = ({ message = 'Loading...' }) => {
  return (
    <div className="flex items-center justify-center py-16">
      <div className="text-center">
        <div className="relative inline-flex">
          <div className="w-16 h-16 rounded-full bg-gradient-to-r from-blue-600 to-indigo-600 opacity-20 animate-ping"></div>
          <div className="absolute top-0 left-0 w-16 h-16 rounded-full border-4 border-transparent border-t-blue-600 animate-spin"></div>
        </div>
        <p className="mt-6 text-gray-600 font-medium">{message}</p>
      </div>
    </div>
  );
};
