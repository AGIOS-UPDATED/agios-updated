'use client';

import { type FC } from 'react';
import { useStore } from '@/store';

interface StarterTemplatesProps {
  onSelect: (template: string) => void;
}

export const StarterTemplates: FC<StarterTemplatesProps> = ({ onSelect }) => {
  const { starterTemplates } = useStore();

  if (!starterTemplates?.length) {
    return null;
  }

  return (
    <div className="space-y-4">
      <h3 className="text-sm font-medium text-gray-900">Starter templates</h3>
      <div className="grid grid-cols-1 gap-3">
        {starterTemplates.map((template, index) => (
          <button
            key={index}
            onClick={() => onSelect(template)}
            className="text-left p-3 bg-white border border-gray-200 rounded-lg hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <p className="text-sm text-gray-600">{template}</p>
          </button>
        ))}
      </div>
    </div>
  );
};
