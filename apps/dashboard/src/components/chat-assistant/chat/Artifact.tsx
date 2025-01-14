'use client';

import { type FC, useState } from 'react';
import { FiDownload, FiEye, FiCopy, FiMaximize2, FiMinimize2 } from 'react-icons/fi';
import { CodeBlock } from './CodeBlock';
import { classNames } from '@/utils/chat-assistant/classNames';

interface ArtifactProps {
  type: 'code' | 'image' | 'file';
  content: string;
  filename?: string;
  language?: string;
  metadata?: Record<string, any>;
}

export const Artifact: FC<ArtifactProps> = ({
  type,
  content,
  filename,
  language,
  metadata,
}) => {
  const [isExpanded, setIsExpanded] = useState(false);
  const [isCopied, setIsCopied] = useState(false);

  const handleDownload = () => {
    const blob = new Blob([content], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename || 'artifact.txt';
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  };

  const handleCopy = async () => {
    try {
      await navigator.clipboard.writeText(content);
      setIsCopied(true);
      setTimeout(() => setIsCopied(false), 2000);
    } catch (error) {
      console.error('Failed to copy content:', error);
    }
  };

  const handleToggleExpand = () => {
    setIsExpanded(!isExpanded);
  };

  const renderContent = () => {
    switch (type) {
      case 'code':
        return (
          <div
            className={classNames(
              'relative group rounded-lg overflow-hidden',
              isExpanded ? 'h-auto' : 'max-h-96'
            )}
          >
            <div className="absolute top-2 right-2 flex space-x-2 opacity-0 group-hover:opacity-100 transition-opacity">
              <button
                onClick={handleCopy}
                className="p-1.5 rounded-md text-gray-500 hover:text-gray-700 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500"
                title={isCopied ? 'Copied!' : 'Copy code'}
              >
                <FiCopy className="w-4 h-4" />
              </button>
              <button
                onClick={handleDownload}
                className="p-1.5 rounded-md text-gray-500 hover:text-gray-700 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500"
                title="Download code"
              >
                <FiDownload className="w-4 h-4" />
              </button>
              <button
                onClick={handleToggleExpand}
                className="p-1.5 rounded-md text-gray-500 hover:text-gray-700 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500"
                title={isExpanded ? 'Collapse' : 'Expand'}
              >
                {isExpanded ? (
                  <FiMinimize2 className="w-4 h-4" />
                ) : (
                  <FiMaximize2 className="w-4 h-4" />
                )}
              </button>
            </div>
            <CodeBlock
              code={content}
              language={language || 'plaintext'}
              showLineNumbers
            />
          </div>
        );

      case 'image':
        return (
          <div className="relative group">
            <img
              src={content}
              alt={filename || 'Generated image'}
              className="max-w-full h-auto rounded-lg"
            />
            <div className="absolute top-2 right-2 flex space-x-2 opacity-0 group-hover:opacity-100 transition-opacity">
              <button
                onClick={handleDownload}
                className="p-1.5 rounded-md bg-white text-gray-700 hover:text-gray-900 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500 shadow-sm"
                title="Download image"
              >
                <FiDownload className="w-4 h-4" />
              </button>
              <button
                onClick={handleToggleExpand}
                className="p-1.5 rounded-md bg-white text-gray-700 hover:text-gray-900 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500 shadow-sm"
                title={isExpanded ? 'View smaller' : 'View larger'}
              >
                {isExpanded ? (
                  <FiMinimize2 className="w-4 h-4" />
                ) : (
                  <FiMaximize2 className="w-4 h-4" />
                )}
              </button>
            </div>
          </div>
        );

      case 'file':
        return (
          <div className="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
            <div className="flex items-center space-x-4">
              <div className="flex-shrink-0">
                <FiEye className="w-6 h-6 text-gray-400" />
              </div>
              <div>
                <div className="text-sm font-medium text-gray-900">
                  {filename || 'Unnamed file'}
                </div>
                {metadata?.size && (
                  <div className="text-sm text-gray-500">
                    Size: {formatFileSize(metadata.size)}
                  </div>
                )}
              </div>
            </div>
            <button
              onClick={handleDownload}
              className="p-1.5 rounded-md text-gray-500 hover:text-gray-700 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-blue-500"
              title="Download file"
            >
              <FiDownload className="w-4 h-4" />
            </button>
          </div>
        );

      default:
        return null;
    }
  };

  return (
    <div
      className={classNames(
        'artifact',
        isExpanded && 'artifact-expanded',
        type === 'code' && 'artifact-code',
        type === 'image' && 'artifact-image',
        type === 'file' && 'artifact-file'
      )}
    >
      {renderContent()}
    </div>
  );
};

function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${parseFloat((bytes / Math.pow(k, i)).toFixed(2))} ${sizes[i]}`;
}
