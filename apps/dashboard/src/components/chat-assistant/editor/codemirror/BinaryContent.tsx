'use client';

import { type FC } from 'react';

interface BinaryContentProps {
  filename: string;
}

export const BinaryContent: FC<BinaryContentProps> = ({ filename }) => {
  return (
    <div className="flex items-center justify-center h-full p-4 bg-gray-100 text-gray-600">
      <p>Binary file: {filename}</p>
    </div>
  );
};
