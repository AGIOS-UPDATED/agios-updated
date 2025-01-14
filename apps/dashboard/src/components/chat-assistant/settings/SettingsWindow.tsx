'use client';

import { type FC, useState } from 'react';
import { FiX } from 'react-icons/fi';
import { Tab } from '@headlessui/react';
import { clsx } from 'clsx';

interface SettingsWindowProps {
  onClose: () => void;
}

const tabs = [
  { name: 'Providers', component: () => <div>Providers Settings</div> },
  { name: 'Features', component: () => <div>Features Settings</div> },
  { name: 'Data', component: () => <div>Data Settings</div> },
  { name: 'Connections', component: () => <div>Connections Settings</div> },
  { name: 'Debug', component: () => <div>Debug Settings</div> },
  { name: 'Event Logs', component: () => <div>Event Logs</div> },
];

export const SettingsWindow: FC<SettingsWindowProps> = ({ onClose }) => {
  const [selectedIndex, setSelectedIndex] = useState(0);

  return (
    <div className="fixed inset-0 z-50 overflow-y-auto">
      <div className="flex min-h-screen items-center justify-center p-4">
        <div className="fixed inset-0 bg-black bg-opacity-25" onClick={onClose} />

        <div className="relative bg-white rounded-lg shadow-xl max-w-4xl w-full max-h-[90vh] flex flex-col">
          {/* Header */}
          <div className="flex items-center justify-between p-4 border-b">
            <h2 className="text-lg font-semibold">Settings</h2>
            <button
              onClick={onClose}
              className="p-2 rounded-full hover:bg-gray-100 transition-colors"
            >
              <FiX className="w-5 h-5" />
            </button>
          </div>

          {/* Content */}
          <div className="flex flex-1 overflow-hidden">
            <Tab.Group
              selectedIndex={selectedIndex}
              onChange={setSelectedIndex}
              vertical
            >
              {/* Sidebar */}
              <div className="w-48 border-r">
                <Tab.List className="flex flex-col py-2">
                  {tabs.map((tab) => (
                    <Tab
                      key={tab.name}
                      className={({ selected }) =>
                        clsx(
                          'px-4 py-2 text-sm text-left focus:outline-none',
                          selected
                            ? 'bg-blue-50 text-blue-600'
                            : 'text-gray-600 hover:bg-gray-50'
                        )
                      }
                    >
                      {tab.name}
                    </Tab>
                  ))}
                </Tab.List>
              </div>

              {/* Tab Content */}
              <div className="flex-1 overflow-y-auto p-6">
                <Tab.Panels>
                  {tabs.map((tab, idx) => (
                    <Tab.Panel
                      key={idx}
                      className={clsx(
                        'focus:outline-none',
                        selectedIndex === idx ? 'block' : 'hidden'
                      )}
                    >
                      <tab.component />
                    </Tab.Panel>
                  ))}
                </Tab.Panels>
              </div>
            </Tab.Group>
          </div>
        </div>
      </div>
    </div>
  );
};
