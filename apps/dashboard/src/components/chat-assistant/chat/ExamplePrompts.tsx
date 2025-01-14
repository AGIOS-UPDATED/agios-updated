'use client';

import { type FC } from 'react';
import { useStore } from '@/store';

interface ExamplePromptsProps {
  onSelect: (prompt: string) => void;
}

export const ExamplePrompts: FC<ExamplePromptsProps> = ({ onSelect }) => {
  const { examplePrompts } = useStore();

  if (!examplePrompts?.length) {
    return null;
  }

  return (
    <div className="space-y-4">
      <h3 className="text-sm font-medium text-gray-900">Example prompts</h3>
      <div className="grid grid-cols-1 gap-3">
        {examplePrompts.map((prompt:any, index:number) => (
          <button
            key={index}
            onClick={() => onSelect(prompt)}
            className="text-left p-3 bg-white border border-gray-200 rounded-lg hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <p className="text-sm text-gray-600">{prompt}</p>
          </button>
        ))}
      </div>
    </div>
  );
};
