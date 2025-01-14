'use client';

import { type FC, useState } from 'react';
import { FiRefreshCw, FiDownload, FiCopy, FiCheck } from 'react-icons/fi';

interface SystemInfo {
  os: string;
  arch: string;
  cpuCores: number;
  totalMemory: string;
  freeMemory: string;
  nodeVersion: string;
  v8Version: string;
}

interface PerformanceMetrics {
  cpuUsage: number;
  memoryUsage: string;
  activeConnections: number;
  requestsPerSecond: number;
  averageResponseTime: number;
}

interface DebugTabProps {
  systemInfo: SystemInfo;
  performanceMetrics: PerformanceMetrics;
  logs: string[];
  onRefreshMetrics: () => void;
  onClearLogs: () => void;
  onExportLogs: () => void;
}

export const DebugTab: FC<DebugTabProps> = ({
  systemInfo,
  performanceMetrics,
  logs,
  onRefreshMetrics,
  onClearLogs,
  onExportLogs,
}) => {
  const [copied, setCopied] = useState(false);
  const [selectedLogLevel, setSelectedLogLevel] = useState('all');
  const [searchQuery, setSearchQuery] = useState('');

  const handleCopyLogs = async () => {
    try {
      await navigator.clipboard.writeText(logs.join('\n'));
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch (err) {
      console.error('Failed to copy logs:', err);
    }
  };

  const filteredLogs = logs.filter((log) => {
    if (selectedLogLevel !== 'all' && !log.toLowerCase().includes(selectedLogLevel)) {
      return false;
    }
    if (searchQuery && !log.toLowerCase().includes(searchQuery.toLowerCase())) {
      return false;
    }
    return true;
  });

  return (
    <div className="space-y-8">
      {/* System Information */}
      <div>
        <h3 className="text-lg font-medium leading-6 text-gray-900 mb-4">
          System Information
        </h3>
        <div className="bg-white shadow overflow-hidden sm:rounded-lg">
          <div className="border-t border-gray-200 px-4 py-5 sm:p-0">
            <dl className="sm:divide-y sm:divide-gray-200">
              <div className="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                <dt className="text-sm font-medium text-gray-500">
                  Operating System
                </dt>
                <dd className="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                  {systemInfo.os}
                </dd>
              </div>
              <div className="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                <dt className="text-sm font-medium text-gray-500">Architecture</dt>
                <dd className="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                  {systemInfo.arch}
                </dd>
              </div>
              <div className="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                <dt className="text-sm font-medium text-gray-500">CPU Cores</dt>
                <dd className="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                  {systemInfo.cpuCores}
                </dd>
              </div>
              <div className="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                <dt className="text-sm font-medium text-gray-500">Total Memory</dt>
                <dd className="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                  {systemInfo.totalMemory}
                </dd>
              </div>
              <div className="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                <dt className="text-sm font-medium text-gray-500">Free Memory</dt>
                <dd className="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                  {systemInfo.freeMemory}
                </dd>
              </div>
              <div className="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                <dt className="text-sm font-medium text-gray-500">Node Version</dt>
                <dd className="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                  {systemInfo.nodeVersion}
                </dd>
              </div>
              <div className="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                <dt className="text-sm font-medium text-gray-500">V8 Version</dt>
                <dd className="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                  {systemInfo.v8Version}
                </dd>
              </div>
            </dl>
          </div>
        </div>
      </div>

      {/* Performance Metrics */}
      <div>
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-lg font-medium leading-6 text-gray-900">
            Performance Metrics
          </h3>
          <button
            onClick={onRefreshMetrics}
            className="inline-flex items-center px-3 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
          >
            <FiRefreshCw className="mr-2 -ml-1 h-4 w-4" />
            Refresh
          </button>
        </div>
        <div className="bg-white shadow overflow-hidden sm:rounded-lg">
          <div className="border-t border-gray-200 px-4 py-5 sm:p-0">
            <dl className="sm:divide-y sm:divide-gray-200">
              <div className="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                <dt className="text-sm font-medium text-gray-500">CPU Usage</dt>
                <dd className="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                  {performanceMetrics.cpuUsage}%
                </dd>
              </div>
              <div className="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                <dt className="text-sm font-medium text-gray-500">
                  Memory Usage
                </dt>
                <dd className="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                  {performanceMetrics.memoryUsage}
                </dd>
              </div>
              <div className="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                <dt className="text-sm font-medium text-gray-500">
                  Active Connections
                </dt>
                <dd className="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                  {performanceMetrics.activeConnections}
                </dd>
              </div>
              <div className="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                <dt className="text-sm font-medium text-gray-500">
                  Requests per Second
                </dt>
                <dd className="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                  {performanceMetrics.requestsPerSecond}
                </dd>
              </div>
              <div className="py-4 sm:py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                <dt className="text-sm font-medium text-gray-500">
                  Average Response Time
                </dt>
                <dd className="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                  {performanceMetrics.averageResponseTime}ms
                </dd>
              </div>
            </dl>
          </div>
        </div>
      </div>

      {/* Logs */}
      <div>
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-lg font-medium leading-6 text-gray-900">Logs</h3>
          <div className="flex space-x-4">
            <button
              onClick={onExportLogs}
              className="inline-flex items-center px-3 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
            >
              <FiDownload className="mr-2 -ml-1 h-4 w-4" />
              Export
            </button>
            <button
              onClick={handleCopyLogs}
              className="inline-flex items-center px-3 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
            >
              {copied ? (
                <FiCheck className="mr-2 -ml-1 h-4 w-4" />
              ) : (
                <FiCopy className="mr-2 -ml-1 h-4 w-4" />
              )}
              {copied ? 'Copied!' : 'Copy'}
            </button>
            <button
              onClick={onClearLogs}
              className="inline-flex items-center px-3 py-2 border border-transparent text-sm font-medium rounded-md shadow-sm text-white bg-red-600 hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500"
            >
              Clear
            </button>
          </div>
        </div>

        <div className="space-y-4">
          <div className="flex space-x-4">
            <div className="flex-1">
              <label
                htmlFor="search"
                className="block text-sm font-medium text-gray-700"
              >
                Search Logs
              </label>
              <input
                type="text"
                id="search"
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
                placeholder="Search..."
              />
            </div>
            <div>
              <label
                htmlFor="level"
                className="block text-sm font-medium text-gray-700"
              >
                Log Level
              </label>
              <select
                id="level"
                value={selectedLogLevel}
                onChange={(e) => setSelectedLogLevel(e.target.value)}
                className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
              >
                <option value="all">All Levels</option>
                <option value="info">Info</option>
                <option value="warn">Warning</option>
                <option value="error">Error</option>
                <option value="debug">Debug</option>
              </select>
            </div>
          </div>

          <div className="bg-gray-900 rounded-lg p-4 overflow-auto max-h-96">
            <pre className="text-gray-100 text-sm font-mono whitespace-pre-wrap">
              {filteredLogs.map((log, index) => (
                <div
                  key={index}
                  className={`py-1 ${
                    log.toLowerCase().includes('error')
                      ? 'text-red-400'
                      : log.toLowerCase().includes('warn')
                      ? 'text-yellow-400'
                      : log.toLowerCase().includes('debug')
                      ? 'text-blue-400'
                      : 'text-gray-100'
                  }`}
                >
                  {log}
                </div>
              ))}
            </pre>
          </div>
        </div>
      </div>
    </div>
  );
};
