'use client';

import { type FC, useRef, useState } from 'react';
import { FiFolder, FiLoader } from 'react-icons/fi';
import { useStore } from '@/store';

interface ImportFolderButtonProps {
  onSuccess?: () => void;
  onError?: (error: Error) => void;
}

export const ImportFolderButton: FC<ImportFolderButtonProps> = ({
  onSuccess,
  onError,
}) => {
  const [isImporting, setIsImporting] = useState(false);
  const directoryInputRef = useRef<HTMLInputElement>(null);
  const { importFolder } = useStore();

  const handleImport = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const directory = e.target.files?.[0];
    if (!directory) return;

    setIsImporting(true);
    try {
      await importFolder(directory);
      onSuccess?.();
    } catch (error) {
      console.error('Failed to import folder:', error);
      onError?.(error as Error);
    } finally {
      setIsImporting(false);
      // Reset the input
      if (directoryInputRef.current) {
        directoryInputRef.current.value = '';
      }
    }
  };

  const handleClick = () => {
    directoryInputRef.current?.click();
  };

  return (
    <>
      <input
        ref={directoryInputRef}
        type="file"
        // @ts-ignore - webkitdirectory is not in the types
        webkitdirectory=""
        // @ts-ignore - directory is not in the types
        directory=""
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
            <FiFolder className="mr-2 -ml-0.5 h-4 w-4" />
            Import Folder
          </>
        )}
      </button>
    </>
  );
};
