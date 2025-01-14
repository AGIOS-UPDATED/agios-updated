'use client';

import { type FC } from 'react';
import { FiDownload } from 'react-icons/fi';
import { useStore } from '@/store';

interface ExportChatButtonProps {
  chatId: string;
}

export const ExportChatButton: FC<ExportChatButtonProps> = ({ chatId }) => {
  const { exportChat } = useStore();

  const handleExport = () => {
    exportChat(chatId);
  };

  return (
    <button
      onClick={handleExport}
      className="inline-flex items-center px-3 py-2 border border-gray-300 shadow-sm text-sm leading-4 font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
    >
      <FiDownload className="mr-2 -ml-0.5 h-4 w-4" />
      Export Chat
    </button>
  );
};
