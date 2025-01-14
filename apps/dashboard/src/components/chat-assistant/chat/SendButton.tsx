'use client';

import { type FC } from 'react';
import { FiSend } from 'react-icons/fi';

interface SendButtonProps {
  onClick: () => void;
  disabled?: boolean;
  isLoading?: boolean;
}

export const SendButton: FC<SendButtonProps> = ({
  onClick,
  disabled = false,
  isLoading = false,
}) => {
  return (
    <button
      onClick={onClick}
      disabled={disabled || isLoading}
      className={`
        inline-flex items-center justify-center p-2 rounded-full
        ${
          disabled
            ? 'bg-gray-300 cursor-not-allowed'
            : 'bg-blue-500 hover:bg-blue-600'
        }
        text-white transition-colors duration-200
      `}
      title="Send message"
    >
      {isLoading ? (
        <div className="animate-spin rounded-full h-5 w-5 border-b-2 border-white" />
      ) : (
        <FiSend size={20} />
      )}
    </button>
  );
};
