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

      // Provider configurations to test
      const providerTests = [
        { 
          name: 'openai', 
          keyName: 'OPENAI_API_KEY',
          displayName: 'OpenAI'
        },
        { 
          name: 'anthropic', 
          keyName: 'ANTHROPIC_API_KEY',
          displayName: 'Anthropic'
        },
        { 
          name: 'google', 
          keyName: 'GOOGLE_API_KEY',
          displayName: 'Google'
        },
        { 
          name: 'groq', 
          keyName: 'GROQ_API_KEY',
          displayName: 'Groq'
        },
        { 
          name: 'cohere', 
          keyName: 'COHERE_API_KEY',
          displayName: 'Cohere'
        },
        { 
          name: 'mistral', 
          keyName: 'MISTRAL_API_KEY',
          displayName: 'Mistral'
        },
      ];

      const results: TestResult[] = [];
      let firstSuccessfulProvider: { name: string; model: string } | null = null;

      for (const test of providerTests) {
        // Check if this provider is available in the system
        const provider = availableProviders.find((p: any) => 
          p.name.toLowerCase() === test.name.toLowerCase()
        );

        if (!provider) {
          continue; // Skip providers that aren't available
        }

        try {
          // Set the API key for this provider
          await upsertConfig({
            body: {
              key: test.keyName,
              value: apiKey,
              is_secret: true
            }
          });

          // Small delay to ensure config is set
          await new Promise(resolve => setTimeout(resolve, 100));

          // Try to get models for this provider
          const modelsResponse = await getProviderModels({
            path: { name: test.name }
          });

          if (modelsResponse.data && modelsResponse.data.length > 0) {
            const firstModel = modelsResponse.data[0];
            results.push({
              provider: test.displayName,
              success: true,
              model: firstModel,
            });

            // Store the first successful provider
            if (!firstSuccessfulProvider) {
              firstSuccessfulProvider = { name: test.name, model: firstModel };
            }
          } else {
            results.push({
              provider: test.displayName,
              success: false,
              error: 'No models available',
            });
          }
        } catch (error: any) {
          let errorMessage = 'Authentication failed';
          
          // Try to extract more specific error information
          if (error?.response?.data?.message) {
            errorMessage = error.response.data.message;
          } else if (error?.message) {
            errorMessage = error.message;
          }

          results.push({
            provider: test.displayName,
            success: false,
            error: errorMessage,
          });
        }
      }

      setTestResults(results);

      // Configure the first successful provider
      if (firstSuccessfulProvider) {
        try {
          await upsertConfig({
            body: {
              key: 'GOOSE_PROVIDER',
              value: firstSuccessfulProvider.name,
              is_secret: false
            }
          });

          await upsertConfig({
            body: {
              key: 'GOOSE_MODEL',
              value: firstSuccessfulProvider.model,
              is_secret: false
            }
          });

          toastService.success({
            title: 'Success!',
            msg: `Configured ${firstSuccessfulProvider.name} with model ${firstSuccessfulProvider.model}`,
          });

          onSuccess(firstSuccessfulProvider.name, firstSuccessfulProvider.model);
        } catch (configError) {
          console.error('Error configuring provider:', configError);
          toastService.error({
            title: 'Configuration Error',
            msg: 'API key validated but failed to configure provider.',
          });
        }
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
            onKeyDown={(e) => {
              if (e.key === 'Enter' && !isLoading && apiKey.trim()) {
                testApiKey();
              }
            }}
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
                      ? 'bg-green-50 text-green-800 border border-green-200 dark:bg-green-900/20 dark:text-green-200 dark:border-green-800'
                      : 'bg-red-50 text-red-800 border border-red-200 dark:bg-red-900/20 dark:text-red-200 dark:border-red-800'
                  }`}
                >
                  <span>{result.success ? '✅' : '❌'}</span>
                  <span className="font-medium">{result.provider}</span>
                  {result.success && result.model && (
                    <span className="text-green-600 dark:text-green-400">- {result.model}</span>
                  )}
                  {!result.success && result.error && (
                    <span className="text-red-600 dark:text-red-400">- {result.error}</span>
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
