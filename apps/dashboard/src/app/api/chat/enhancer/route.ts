import { streamText } from '@/lib/llm/stream-text';
import { stripIndents } from '@/utils/chat-assistant/stripIndent';
import type { IProviderSetting, ProviderInfo } from '@/types/model';

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
    const { message, model, provider } = await request.json();
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

    const cookieHeader = request.headers.get('Cookie');
    const apiKeys = JSON.parse(parseCookies(cookieHeader || '').apiKeys || '{}');
    const providerSettings: Record<string, IProviderSetting> = JSON.parse(
      parseCookies(cookieHeader || '').providers || '{}'
    );

    const prompt = stripIndents`
      You are an AI writing assistant. Your task is to enhance the given text while maintaining its original meaning and intent.
      Make it more professional, clear, and engaging. Focus on:
      1. Improving clarity and readability
      2. Enhancing grammar and word choice
      3. Maintaining the original tone and intent
      4. Making it more concise when possible

      Text to enhance:
      ${message}

      Enhanced version:
    `;

    const textStream = await streamText(
      [{ role: 'user', content: prompt }],
      {
        maxTokens: 2000,
        apiKeys,
        providerSettings,
        model,
        provider,
      }
    );

    return new Response(textStream);
  } catch (error) {
    console.error('Error in enhancer API:', error);
    return new Response(JSON.stringify({ error: 'Internal Server Error' }), {
      status: 500,
      headers: {
        'Content-Type': 'application/json',
      },
    });
  }
}
