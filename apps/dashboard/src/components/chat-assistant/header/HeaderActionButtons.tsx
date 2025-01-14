'use client';

import { type FC } from 'react';
import { FiSettings, FiGithub, FiHelpCircle } from 'react-icons/fi';
import Link from 'next/link';

interface HeaderActionButtonsProps {
  onSettingsClick: () => void;
  onHelpClick: () => void;
}

export const HeaderActionButtons: FC<HeaderActionButtonsProps> = ({
  onSettingsClick,
  onHelpClick,
}) => {
  return (
    <div className="flex items-center space-x-2">
      <button
        onClick={onHelpClick}
        className="p-2 rounded-full hover:bg-gray-100 text-gray-600 hover:text-gray-800 transition-colors"
        title="Help"
      >
        <FiHelpCircle className="w-5 h-5" />
      </button>

      <button
        onClick={onSettingsClick}
        className="p-2 rounded-full hover:bg-gray-100 text-gray-600 hover:text-gray-800 transition-colors"
        title="Settings"
      >
        <FiSettings className="w-5 h-5" />
      </button>

      <Link
        href="https://github.com/codeium/cascade"
        target="_blank"
        rel="noopener noreferrer"
        className="p-2 rounded-full hover:bg-gray-100 text-gray-600 hover:text-gray-800 transition-colors"
        title="GitHub"
      >
        <FiGithub className="w-5 h-5" />
      </Link>
    </div>
  );
};
