'use client';

import { type FC, useState } from 'react';
import { FiDownload, FiUpload, FiTrash2, FiAlertCircle } from 'react-icons/fi';
import { Switch } from '@headlessui/react';
import { clsx } from 'clsx';

interface DataSettings {
  autoSave: boolean;
  autoBackup: boolean;
  backupInterval: number;
  maxBackups: number;
  compressionEnabled: boolean;
  encryptionEnabled: boolean;
  storageLocation: string;
}

interface DataTabProps {
  settings: DataSettings;
  onSettingsChange: (settings: Partial<DataSettings>) => void;
  onExportData: () => void;
  onImportData: (file: File) => Promise<void>;
  onClearData: () => void;
}

export const DataTab: FC<DataTabProps> = ({
  settings,
  onSettingsChange,
  onExportData,
  onImportData,
  onClearData,
}) => {
  const [showClearConfirm, setShowClearConfirm] = useState(false);
  const [importError, setImportError] = useState<string | null>(null);

  const handleImport = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;

    try {
      setImportError(null);
      await onImportData(file);
    } catch (err) {
      setImportError(
        err instanceof Error ? err.message : 'Failed to import data'
      );
    }
  };

  return (
    <div className="space-y-8">
      {/* General Settings */}
      <div>
        <h3 className="text-lg font-medium leading-6 text-gray-900 mb-4">
          General Settings
        </h3>
        <div className="space-y-6">
          <div className="flex items-center justify-between">
            <div>
              <label className="text-sm font-medium text-gray-700">
                Auto Save
              </label>
              <p className="text-sm text-gray-500">
                Automatically save changes as you work
              </p>
            </div>
            <Switch
              checked={settings.autoSave}
              onChange={(checked) => onSettingsChange({ autoSave: checked })}
              className={clsx(
                settings.autoSave ? 'bg-blue-600' : 'bg-gray-200',
                'relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2'
              )}
            >
              <span className="sr-only">Auto Save</span>
              <span
                className={clsx(
                  settings.autoSave ? 'translate-x-5' : 'translate-x-0',
                  'pointer-events-none relative inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out'
                )}
              />
            </Switch>
          </div>

          <div className="flex items-center justify-between">
            <div>
              <label className="text-sm font-medium text-gray-700">
                Auto Backup
              </label>
              <p className="text-sm text-gray-500">
                Create periodic backups of your data
              </p>
            </div>
            <Switch
              checked={settings.autoBackup}
              onChange={(checked) => onSettingsChange({ autoBackup: checked })}
              className={clsx(
                settings.autoBackup ? 'bg-blue-600' : 'bg-gray-200',
                'relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2'
              )}
            >
              <span className="sr-only">Auto Backup</span>
              <span
                className={clsx(
                  settings.autoBackup ? 'translate-x-5' : 'translate-x-0',
                  'pointer-events-none relative inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out'
                )}
              />
            </Switch>
          </div>

          {settings.autoBackup && (
            <>
              <div>
                <label
                  htmlFor="backupInterval"
                  className="block text-sm font-medium text-gray-700"
                >
                  Backup Interval (minutes)
                </label>
                <input
                  type="number"
                  id="backupInterval"
                  value={settings.backupInterval}
                  onChange={(e) =>
                    onSettingsChange({
                      backupInterval: parseInt(e.target.value),
                    })
                  }
                  min="1"
                  className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                />
              </div>

              <div>
                <label
                  htmlFor="maxBackups"
                  className="block text-sm font-medium text-gray-700"
                >
                  Maximum Backups
                </label>
                <input
                  type="number"
                  id="maxBackups"
                  value={settings.maxBackups}
                  onChange={(e) =>
                    onSettingsChange({
                      maxBackups: parseInt(e.target.value),
                    })
                  }
                  min="1"
                  className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                />
              </div>
            </>
          )}

          <div className="flex items-center justify-between">
            <div>
              <label className="text-sm font-medium text-gray-700">
                Compression
              </label>
              <p className="text-sm text-gray-500">
                Compress data to save storage space
              </p>
            </div>
            <Switch
              checked={settings.compressionEnabled}
              onChange={(checked) =>
                onSettingsChange({ compressionEnabled: checked })
              }
              className={clsx(
                settings.compressionEnabled ? 'bg-blue-600' : 'bg-gray-200',
                'relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2'
              )}
            >
              <span className="sr-only">Compression</span>
              <span
                className={clsx(
                  settings.compressionEnabled ? 'translate-x-5' : 'translate-x-0',
                  'pointer-events-none relative inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out'
                )}
              />
            </Switch>
          </div>

          <div className="flex items-center justify-between">
            <div>
              <label className="text-sm font-medium text-gray-700">
                Encryption
              </label>
              <p className="text-sm text-gray-500">
                Encrypt data for additional security
              </p>
            </div>
            <Switch
              checked={settings.encryptionEnabled}
              onChange={(checked) =>
                onSettingsChange({ encryptionEnabled: checked })
              }
              className={clsx(
                settings.encryptionEnabled ? 'bg-blue-600' : 'bg-gray-200',
                'relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2'
              )}
            >
              <span className="sr-only">Encryption</span>
              <span
                className={clsx(
                  settings.encryptionEnabled ? 'translate-x-5' : 'translate-x-0',
                  'pointer-events-none relative inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out'
                )}
              />
            </Switch>
          </div>

          <div>
            <label
              htmlFor="storageLocation"
              className="block text-sm font-medium text-gray-700"
            >
              Storage Location
            </label>
            <input
              type="text"
              id="storageLocation"
              value={settings.storageLocation}
              onChange={(e) =>
                onSettingsChange({ storageLocation: e.target.value })
              }
              className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
            />
          </div>
        </div>
      </div>

      {/* Data Management */}
      <div>
        <h3 className="text-lg font-medium leading-6 text-gray-900 mb-4">
          Data Management
        </h3>
        <div className="space-y-4">
          <div className="flex space-x-4">
            <button
              onClick={onExportData}
              className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
            >
              <FiDownload className="mr-2 -ml-1 h-4 w-4" />
              Export Data
            </button>

            <label className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 cursor-pointer">
              <FiUpload className="mr-2 -ml-1 h-4 w-4" />
              Import Data
              <input
                type="file"
                className="hidden"
                onChange={handleImport}
                accept=".json,.zip"
              />
            </label>
          </div>

          {importError && (
            <div className="rounded-md bg-red-50 p-4">
              <div className="flex">
                <div className="flex-shrink-0">
                  <FiAlertCircle
                    className="h-5 w-5 text-red-400"
                    aria-hidden="true"
                  />
                </div>
                <div className="ml-3">
                  <h3 className="text-sm font-medium text-red-800">
                    Import Error
                  </h3>
                  <div className="mt-2 text-sm text-red-700">{importError}</div>
                </div>
              </div>
            </div>
          )}

          {!showClearConfirm ? (
            <button
              onClick={() => setShowClearConfirm(true)}
              className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500"
            >
              <FiTrash2 className="mr-2 -ml-1 h-4 w-4" />
              Clear All Data
            </button>
          ) : (
            <div className="space-y-4">
              <p className="text-sm text-red-600">
                Are you sure you want to clear all data? This action cannot be
                undone.
              </p>
              <div className="flex space-x-4">
                <button
                  onClick={onClearData}
                  className="inline-flex items-center px-4 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500"
                >
                  Yes, Clear Data
                </button>
                <button
                  onClick={() => setShowClearConfirm(false)}
                  className="inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md shadow-sm text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                >
                  Cancel
                </button>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
