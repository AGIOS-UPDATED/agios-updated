import { type FC } from 'react';
import { Markdown } from './Markdown';

interface UserMessageProps {
  content: string;
  timestamp?: string;
}

export const UserMessage: FC<UserMessageProps> = ({ content, timestamp }) => {
  return (
    <div className="flex flex-col space-y-2 p-4 bg-gray-100 rounded-lg">
      <div className="flex items-center space-x-2">
        <div className="w-8 h-8 rounded-full bg-blue-500 flex items-center justify-center text-white">
          U
        </div>
        {timestamp && (
          <span className="text-sm text-gray-500">{timestamp}</span>
        )}
      </div>
      <div className="pl-10">
        <Markdown content={content} />
      </div>
    </div>
  );
};
