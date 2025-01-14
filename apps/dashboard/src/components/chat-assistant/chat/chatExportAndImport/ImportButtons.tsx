'use client';

import { type FC, useRef, useState } from 'react';
import { FiUpload, FiLoader } from 'react-icons/fi';
import { useStore } from '@/store';

interface ImportButtonsProps {
  onSuccess?: () => void;
  onError?: (error: Error) => void;
}

export const ImportButtons: FC<ImportButtonsProps> = ({
  onSuccess,
  onError,
}) => {
  const [isImporting, setIsImporting] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);
  const { importChat } = useStore();

  const handleImport = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    setIsImporting(true);
    try {
      const reader = new FileReader();
      reader.onload = async (event) => {
        try {
          const content = event.target?.result;
          if (typeof content !== 'string') {
            throw new Error('Invalid file content');
          }

          await importChat(content);
          onSuccess?.();
        } catch (error) {
          console.error('Failed to import chat:', error);
          onError?.(error as Error);
        } finally {
          setIsImporting(false);
          // Reset the input
          if (fileInputRef.current) {
            fileInputRef.current.value = '';
          }
        }
      };

      reader.onerror = () => {
        const error = new Error('Failed to read file');
        console.error(error);
        onError?.(error);
        setIsImporting(false);
      };

      reader.readAsText(file);
    } catch (error) {
      console.error('Failed to import chat:', error);
      onError?.(error as Error);
      setIsImporting(false);
    }
  };

  const handleClick = () => {
    fileInputRef.current?.click();
  };

  return (
    <>
      <input
        ref={fileInputRef}
        type="file"
        accept=".json"
        className="hidden"
        onChange={handleImport}
        disabled={isImporting}
      />
      <button
        onClick={handleClick}
        disabled={isImporting}
        className="inline-flex items-center px-3 py-2 border border-gray-300 shadow-sm text-sm leading-4 font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        {isImporting ? (
          <>
            <FiLoader className="animate-spin mr-2 -ml-0.5 h-4 w-4" />
            Importing...
          </>
        ) : (
          <>
            <FiUpload className="mr-2 -ml-0.5 h-4 w-4" />
            Import Chat
          </>
        )}
      </button>
    </>
  );
};
