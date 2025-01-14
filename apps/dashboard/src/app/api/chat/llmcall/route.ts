import { streamText } from '@/lib/llm/stream-text';
import type { IProviderSetting, ProviderInfo } from '@/types/model';
import { generateText } from 'ai';
import { getModelList, PROVIDER_LIST } from '@/utils/constants';
import { MAX_TOKENS } from '@/lib/constants';

function parseCookies(cookieHeader: string) {
  const cookies: any = {};
  if (!cookieHeader) return cookies;

  const items = cookieHeader.split(';').map((cookie) => cookie.trim());

  items.forEach((item) => {
    const [name, ...rest] = item.split('=');
    if (name && rest) {
      const decodedName = decodeURIComponent(name.trim());
      const decodedValue = decodeURIComponent(rest.join('=').trim());
      cookies[decodedName] = decodedValue;
    }
  });

  return cookies;
}

export async function POST(request: Request) {
  try {
    const { system, message, model, provider, streamOutput } = await request.json();
    const { name: providerName } = provider;

    // Validate 'model' and 'provider' fields
    if (!model || typeof model !== 'string') {
      return new Response('Invalid or missing model', {
        status: 400,
        statusText: 'Bad Request',
      });
    }

    if (!providerName || typeof providerName !== 'string') {
      return new Response('Invalid or missing provider', {
        status: 400,
        statusText: 'Bad Request',
      });
    }

    // Check if the provider is supported
    if (!PROVIDER_LIST.some((p) => p.name === providerName)) {
      return new Response('Unsupported provider', {
        status: 400,
        statusText: 'Bad Request',
      });
    }

    // Check if the model is supported for the provider
    const supportedModels = getModelList(providerName);
    if (!supportedModels.some((m) => m.id === model)) {
      return new Response('Unsupported model for the provider', {
        status: 400,
        statusText: 'Bad Request',
      });
    }

    const cookieHeader = request.headers.get('Cookie');
    const apiKeys = JSON.parse(parseCookies(cookieHeader || '').apiKeys || '{}');
    const providerSettings: Record<string, IProviderSetting> = JSON.parse(
      parseCookies(cookieHeader || '').providers || '{}'
    );

    const messages = [
      { role: 'system', content: system },
      { role: 'user', content: message }
    ];

    if (streamOutput) {
      const textStream = await streamText(messages, {
        maxTokens: MAX_TOKENS,
        apiKeys,
        providerSettings,
        model,
        provider,
      });

      return new Response(textStream);
    } else {
      const response = await generateText({
        messages,
        maxTokens: MAX_TOKENS,
        apiKeys,
        providerSettings,
        model,
        provider,
      });

      return Response.json({ response });
    }
  } catch (error) {
    console.error('Error in llmcall API:', error);
    return Response.json({ error: 'Internal Server Error' }, { status: 500 });
  }
}
