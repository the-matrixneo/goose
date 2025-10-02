import { useState } from 'react';
import { providers, getProviderModels, upsertConfig } from '../api';
import { useConfig } from './ConfigContext';
import { toastService } from '../toasts';

interface ApiKeyTesterProps {
  onSuccess: (provider: string, model: string) => void;
}

interface TestResult {
  provider: string;
  success: boolean;
  model?: string;
  error?: string;
}

export default function ApiKeyTester({ onSuccess }: ApiKeyTesterProps) {
  const [apiKey, setApiKey] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [testResults, setTestResults] = useState<TestResult[]>([]);
  const [showResults, setShowResults] = useState(false);
  const { upsert } = useConfig();

  const testApiKey = async () => {
    if (!apiKey.trim()) {
      toastService.error({
        title: 'API Key Required',
        msg: 'Please enter an API key to test.',
      });
      return;
    }

    setIsLoading(true);
    setTestResults([]);
    setShowResults(true);

    try {
      // Get all available providers
      const providersResponse = await providers();
      const availableProviders = providersResponse.data || [];

      // Common provider patterns to test
      const providerTests = [
        { name: 'openai', keyFormat: 'OPENAI_API_KEY' },
        { name: 'anthropic', keyFormat: 'ANTHROPIC_API_KEY' },
        { name: 'google', keyFormat: 'GOOGLE_API_KEY' },
        { name: 'groq', keyFormat: 'GROQ_API_KEY' },
        { name: 'cohere', keyFormat: 'COHERE_API_KEY' },
        { name: 'mistral', keyFormat: 'MISTRAL_API_KEY' },
      ];

      const results: TestResult[] = [];

      for (const test of providerTests) {
        // Check if this provider is available
        const provider = availableProviders.find((p: any) => 
          p.name.toLowerCase() === test.name.toLowerCase()
        );

        if (!provider) {
          continue;
        }

        try {
          // Temporarily set the API key
          await upsert(test.keyFormat, apiKey, true);

          // Try to get models for this provider
          const modelsResponse = await getProviderModels({
            path: { name: test.name }
          });

          if (modelsResponse.data && modelsResponse.data.length > 0) {
            const firstModel = modelsResponse.data[0];
            results.push({
              provider: test.name,
              success: true,
              model: firstModel,
            });

            // If this is the first successful test, configure it
            if (results.filter(r => r.success).length === 1) {
              await upsert('GOOSE_PROVIDER', test.name, false);
              await upsert('GOOSE_MODEL', firstModel, false);
              
              toastService.success({
                title: 'Success!',
                msg: `Configured ${test.name} with model ${firstModel}`,
              });

              onSuccess(test.name, firstModel);
            }
          } else {
            results.push({
              provider: test.name,
              success: false,
              error: 'No models available',
            });
          }
        } catch (error) {
          results.push({
            provider: test.name,
            success: false,
            error: error instanceof Error ? error.message : 'Authentication failed',
          });
        }
      }

      setTestResults(results);

      if (results.some(r => r.success)) {
        toastService.success({
          title: 'API Key Valid!',
          msg: `Found ${results.filter(r => r.success).length} working provider(s)`,
        });
      } else {
        toastService.error({
          title: 'No Valid Providers',
          msg: 'The API key did not work with any available providers.',
        });
      }

    } catch (error) {
      console.error('Error testing API key:', error);
      toastService.error({
        title: 'Test Failed',
        msg: 'Failed to test API key. Please try again.',
      });
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="w-full p-4 sm:p-6 bg-background-muted border border-background-hover rounded-xl mb-6">
      <div className="flex items-start justify-between mb-3">
        <div className="flex-1">
          <div className="flex items-center gap-2 mb-2">
            <span className="text-lg">🔑</span>
            <h3 className="font-medium text-text-standard text-sm sm:text-base">
              Quick Setup with API Key
            </h3>
          </div>
        </div>
      </div>
      
      <p className="text-text-muted text-sm sm:text-base mb-4">
        Enter your API key and we'll automatically detect which provider it works with.
      </p>

      <div className="space-y-4">
        <div className="flex gap-2">
          <input
            type="password"
            value={apiKey}
            onChange={(e) => setApiKey(e.target.value)}
            placeholder="Enter your API key (OpenAI, Anthropic, Google, etc.)"
            className="flex-1 px-3 py-2 border border-background-hover rounded-lg bg-background-default text-text-standard placeholder-text-muted focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            disabled={isLoading}
          />
          <button
            onClick={testApiKey}
            disabled={isLoading || !apiKey.trim()}
            className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            {isLoading ? (
              <div className="flex items-center gap-2">
                <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
                Testing...
              </div>
            ) : (
              'Test Key'
            )}
          </button>
        </div>

        {showResults && testResults.length > 0 && (
          <div className="space-y-2">
            <h4 className="font-medium text-text-standard text-sm">Test Results:</h4>
            <div className="space-y-1">
              {testResults.map((result, index) => (
                <div
                  key={index}
                  className={`flex items-center gap-2 text-sm p-2 rounded ${
                    result.success
                      ? 'bg-green-50 text-green-800 border border-green-200'
                      : 'bg-red-50 text-red-800 border border-red-200'
                  }`}
                >
                  <span>{result.success ? '✅' : '❌'}</span>
                  <span className="font-medium capitalize">{result.provider}</span>
                  {result.success && result.model && (
                    <span className="text-green-600">- {result.model}</span>
                  )}
                  {!result.success && result.error && (
                    <span className="text-red-600">- {result.error}</span>
                  )}
                </div>
              ))}
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
