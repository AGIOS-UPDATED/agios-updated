import { type FC } from 'react';
import { Markdown } from './Markdown';

interface AssistantMessageProps {
  content: string;
  timestamp?: string;
  isLoading?: boolean;
}

export const AssistantMessage: FC<AssistantMessageProps> = ({
  content,
  timestamp,
  isLoading,
}) => {
  return (
    <div className="flex flex-col space-y-2 p-4 bg-white rounded-lg border border-gray-200">
      <div className="flex items-center space-x-2">
        <div className="w-8 h-8 rounded-full bg-green-500 flex items-center justify-center text-white">
          A
        </div>
        {timestamp && (
          <span className="text-sm text-gray-500">{timestamp}</span>
        )}
        {isLoading && (
          <div className="animate-pulse flex space-x-2">
            <div className="h-2 w-2 bg-gray-500 rounded-full"></div>
            <div className="h-2 w-2 bg-gray-500 rounded-full"></div>
            <div className="h-2 w-2 bg-gray-500 rounded-full"></div>
          </div>
        )}
      </div>
      <div className="pl-10">
        <Markdown content={content} />
      </div>
    </div>
  );
};
