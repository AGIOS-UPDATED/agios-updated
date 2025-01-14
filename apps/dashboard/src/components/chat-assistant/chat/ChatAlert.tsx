'use client';

import { type FC } from 'react';
import { FiAlertCircle, FiX } from 'react-icons/fi';

interface ChatAlertProps {
  message: string;
  type?: 'info' | 'warning' | 'error' | 'success';
  onClose?: () => void;
}

export const ChatAlert: FC<ChatAlertProps> = ({
  message,
  type = 'info',
  onClose,
}) => {
  const getAlertStyles = () => {
    switch (type) {
      case 'error':
        return 'bg-red-50 text-red-800 border-red-200';
      case 'warning':
        return 'bg-yellow-50 text-yellow-800 border-yellow-200';
      case 'success':
        return 'bg-green-50 text-green-800 border-green-200';
      default:
        return 'bg-blue-50 text-blue-800 border-blue-200';
    }
  };

  const getIconColor = () => {
    switch (type) {
      case 'error':
        return 'text-red-500';
      case 'warning':
        return 'text-yellow-500';
      case 'success':
        return 'text-green-500';
      default:
        return 'text-blue-500';
    }
  };

  return (
    <div
      className={`flex items-center p-4 mb-4 rounded-lg border ${getAlertStyles()}`}
      role="alert"
    >
      <FiAlertCircle className={`flex-shrink-0 w-5 h-5 mr-2 ${getIconColor()}`} />
      <span className="flex-1 text-sm font-medium">{message}</span>
      {onClose && (
        <button
          type="button"
          className={`ml-auto -mx-1.5 -my-1.5 rounded-lg focus:ring-2 p-1.5 inline-flex h-8 w-8 ${getAlertStyles()} hover:bg-opacity-75`}
          onClick={onClose}
          aria-label="Close"
        >
          <FiX className="w-5 h-5" />
        </button>
      )}
    </div>
  );
};
