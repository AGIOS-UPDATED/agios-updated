'use client';

import { type FC, useState } from 'react';
import { FiGitBranch, FiLoader } from 'react-icons/fi';
import { useStore } from '@/store';

interface GitCloneButtonProps {
  onSuccess?: () => void;
  onError?: (error: Error) => void;
}

export const GitCloneButton: FC<GitCloneButtonProps> = ({
  onSuccess,
  onError,
}) => {
  const [isCloning, setIsCloning] = useState(false);
  const [repoUrl, setRepoUrl] = useState('');
  const [showInput, setShowInput] = useState(false);
  const { cloneRepository } = useStore();

  const handleClone = async () => {
    if (!repoUrl.trim()) return;

    setIsCloning(true);
    try {
      await cloneRepository(repoUrl);
      setRepoUrl('');
      setShowInput(false);
      onSuccess?.();
    } catch (error) {
      console.error('Failed to clone repository:', error);
      onError?.(error as Error);
    } finally {
      setIsCloning(false);
    }
  };

  if (!showInput) {
    return (
      <button
        onClick={() => setShowInput(true)}
        className="inline-flex items-center px-3 py-2 border border-gray-300 shadow-sm text-sm leading-4 font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
      >
        <FiGitBranch className="mr-2 -ml-0.5 h-4 w-4" />
        Clone Repository
      </button>
    );
  }

  return (
    <div className="flex flex-col space-y-2">
      <div className="flex space-x-2">
        <input
          type="text"
          value={repoUrl}
          onChange={(e) => setRepoUrl(e.target.value)}
          placeholder="Enter repository URL"
          className="flex-1 min-w-0 block w-full px-3 py-2 text-sm border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500"
          disabled={isCloning}
        />
        <button
          onClick={handleClone}
          disabled={!repoUrl.trim() || isCloning}
          className="inline-flex items-center px-3 py-2 border border-transparent text-sm leading-4 font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          {isCloning ? (
            <>
              <FiLoader className="animate-spin mr-2 -ml-0.5 h-4 w-4" />
              Cloning...
            </>
          ) : (
            <>
              <FiGitBranch className="mr-2 -ml-0.5 h-4 w-4" />
              Clone
            </>
          )}
        </button>
        <button
          onClick={() => setShowInput(false)}
          className="inline-flex items-center px-3 py-2 border border-gray-300 text-sm leading-4 font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
          disabled={isCloning}
        >
          Cancel
        </button>
      </div>
      <p className="text-xs text-gray-500">
        Enter the HTTPS URL of the Git repository you want to clone
      </p>
    </div>
  );
};
