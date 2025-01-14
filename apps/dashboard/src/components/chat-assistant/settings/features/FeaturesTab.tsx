'use client';

import { type FC } from 'react';
import { Switch } from '@headlessui/react';
import { clsx } from 'clsx';

interface Feature {
  id: string;
  name: string;
  description: string;
  enabled: boolean;
  category: string;
  experimental?: boolean;
}

interface FeaturesTabProps {
  features: Feature[];
  onToggleFeature: (featureId: string, enabled: boolean) => void;
}

export const FeaturesTab: FC<FeaturesTabProps> = ({
  features,
  onToggleFeature,
}) => {
  const categories = Array.from(
    new Set(features.map((feature) => feature.category))
  );

  return (
    <div className="space-y-8">
      {categories.map((category) => (
        <div key={category}>
          <h3 className="text-lg font-medium leading-6 text-gray-900 mb-4">
            {category}
          </h3>
          <div className="bg-white shadow overflow-hidden sm:rounded-lg">
            <ul className="divide-y divide-gray-200">
              {features
                .filter((feature) => feature.category === category)
                .map((feature) => (
                  <li key={feature.id} className="px-4 py-5 sm:p-6">
                    <div className="flex items-center justify-between">
                      <div className="flex-1 min-w-0 mr-6">
                        <div className="flex items-center">
                          <h4 className="text-sm font-medium text-gray-900">
                            {feature.name}
                          </h4>
                          {feature.experimental && (
                            <span className="ml-2 inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800">
                              Experimental
                            </span>
                          )}
                        </div>
                        <p className="mt-1 text-sm text-gray-500">
                          {feature.description}
                        </p>
                      </div>
                      <Switch
                        checked={feature.enabled}
                        onChange={(checked) =>
                          onToggleFeature(feature.id, checked)
                        }
                        className={clsx(
                          feature.enabled ? 'bg-blue-600' : 'bg-gray-200',
                          'relative inline-flex flex-shrink-0 h-6 w-11 border-2 border-transparent rounded-full cursor-pointer transition-colors ease-in-out duration-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500'
                        )}
                      >
                        <span className="sr-only">
                          {feature.enabled ? 'Enable' : 'Disable'} {feature.name}
                        </span>
                        <span
                          className={clsx(
                            feature.enabled ? 'translate-x-5' : 'translate-x-0',
                            'pointer-events-none relative inline-block h-5 w-5 rounded-full bg-white shadow transform ring-0 transition ease-in-out duration-200'
                          )}
                        >
                          <span
                            className={clsx(
                              feature.enabled
                                ? 'opacity-0 ease-out duration-100'
                                : 'opacity-100 ease-in duration-200',
                              'absolute inset-0 h-full w-full flex items-center justify-center transition-opacity'
                            )}
                            aria-hidden="true"
                          >
                            <svg
                              className="h-3 w-3 text-gray-400"
                              fill="none"
                              viewBox="0 0 12 12"
                            >
                              <path
                                d="M4 8l2-2m0 0l2-2M6 6L4 4m2 2l2 2"
                                stroke="currentColor"
                                strokeWidth={2}
                                strokeLinecap="round"
                                strokeLinejoin="round"
                              />
                            </svg>
                          </span>
                          <span
                            className={clsx(
                              feature.enabled
                                ? 'opacity-100 ease-in duration-200'
                                : 'opacity-0 ease-out duration-100',
                              'absolute inset-0 h-full w-full flex items-center justify-center transition-opacity'
                            )}
                            aria-hidden="true"
                          >
                            <svg
                              className="h-3 w-3 text-blue-600"
                              fill="currentColor"
                              viewBox="0 0 12 12"
                            >
                              <path d="M3.707 5.293a1 1 0 00-1.414 1.414l1.414-1.414zM5 8l-.707.707a1 1 0 001.414 0L5 8zm4.707-3.293a1 1 0 00-1.414-1.414l1.414 1.414zm-7.414 2l2 2 1.414-1.414-2-2-1.414 1.414zm3.414 2l4-4-1.414-1.414-4 4 1.414 1.414z" />
                            </svg>
                          </span>
                        </span>
                      </Switch>
                    </div>
                  </li>
                ))}
            </ul>
          </div>
        </div>
      ))}
    </div>
  );
};
