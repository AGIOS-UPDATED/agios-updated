'use client';

import { type FC } from 'react';
import Image from 'next/image';
import Link from 'next/link';
import { HeaderActionButtons } from './HeaderActionButtons';

interface HeaderProps {
  onSettingsClick: () => void;
  onHelpClick: () => void;
}

export const Header: FC<HeaderProps> = ({ onSettingsClick, onHelpClick }) => {
  return (
    <header className="bg-white border-b border-gray-200">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between items-center h-16">
          <div className="flex items-center">
            <Link href="/" className="flex items-center space-x-2">
              <Image
                src="/logo.svg"
                alt="Cascade Logo"
                width={32}
                height={32}
                priority
              />
              <span className="text-xl font-semibold text-gray-900">Cascade</span>
            </Link>
          </div>

          <div className="flex items-center space-x-4">
            <nav className="hidden md:flex space-x-8">
              <Link
                href="/chat"
                className="text-gray-600 hover:text-gray-900 px-3 py-2 rounded-md text-sm font-medium"
              >
                Chat
              </Link>
              <Link
                href="/docs"
                className="text-gray-600 hover:text-gray-900 px-3 py-2 rounded-md text-sm font-medium"
              >
                Documentation
              </Link>
            </nav>

            <HeaderActionButtons
              onSettingsClick={onSettingsClick}
              onHelpClick={onHelpClick}
            />
          </div>
        </div>
      </div>
    </header>
  );
};
