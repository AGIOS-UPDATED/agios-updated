import { type FC, useState, useEffect } from 'react';
import { PROVIDER_LIST } from '@/utils/constants';

interface Model {
  id: string;
  name: string;
  provider: string;
  description?: string;
}

interface ModelSelectorProps {
  selectedModel: string;
  onModelChange: (model: string, provider: any) => void;
}

export const ModelSelector: FC<ModelSelectorProps> = ({
  selectedModel,
  onModelChange,
}) => {
  const [models, setModels] = useState<Model[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchModels = async () => {
      try {
        const response = await fetch('/api/models');
        if (!response.ok) {
          throw new Error('Failed to fetch models');
        }
        const data = await response.json();
        setModels(data);
      } catch (err) {
        setError(err instanceof Error ? err.message : 'Failed to load models');
      } finally {
        setLoading(false);
      }
    };

    fetchModels();
  }, []);

  const handleModelChange = (modelId: string) => {
    const model = models.find(m => m.id === modelId);

    if (model) {

      const provider = PROVIDER_LIST.find(p => p.name === model.provider);
     
      if (provider) {
        onModelChange(modelId, provider);
      }

    }
  };

  if (loading) {
    return (
      <div className="animate-pulse flex space-x-4">
        <div className="h-10 w-full bg-gray-200 rounded"></div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="text-red-500 text-sm">
        Error loading models: {error}
      </div>
    );
  }

  return (
    <div className="flex flex-col space-y-2">
      <label htmlFor="model-select" className="text-sm font-medium text-gray-700">
        Model
      </label>
      <select
        id="model-select"
        value={selectedModel}
        onChange={(e) => handleModelChange(e.target.value)}
        className="block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 sm:text-sm"
      >
        {models.map((model) => (
          <option key={model.id} value={model.id}>
            {model.name} ({model.provider})
          </option>
        ))}
      </select>
    </div>
  );
};
