import { createDataStream } from 'ai';
import { MAX_RESPONSE_SEGMENTS, MAX_TOKENS } from '@/lib/constants';
import { CONTINUE_PROMPT } from '@/lib/prompts';
import { streamText } from '@/lib/llm/stream-text';
import { SwitchableStream } from '@/lib/llm/switchable-stream';
import { type Messages, type StreamingOptions } from '@/types/chat';
import { type IProviderSetting } from '@/types/model';
import { createScopedLogger } from '@/utils/logger';

const logger = createScopedLogger('api.chat');

function parseCookies(cookieHeader: string): Record<string, string> {
  const cookies: Record<string, string> = {};
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
    const { messages, files, promptId, contextOptimization } = await request.json();

    const cookieHeader = request.headers.get('Cookie');
    const apiKeys = JSON.parse(parseCookies(cookieHeader || '').apiKeys || '{}');
    const providerSettings: Record<string, IProviderSetting> = JSON.parse(
      parseCookies(cookieHeader || '').providers || '{}'
    );

    const stream = new SwitchableStream();

    const cumulativeUsage = {
      completionTokens: 0,
    };

    const streamingOptions: StreamingOptions = {
      maxResponseSegments: MAX_RESPONSE_SEGMENTS,
      maxTokens: MAX_TOKENS,
      onToken: () => {
        cumulativeUsage.completionTokens++;
      },
    };

    const textStream = await streamText(messages, {
      ...streamingOptions,
      apiKeys,
      providerSettings,
      files,
      promptId,
      contextOptimization,
    });

    return new Response(textStream);
  } catch (error) {
    logger.error('Error in chat API:', error);
    return new Response(JSON.stringify({ error: 'Internal Server Error' }), {
      status: 500,
      headers: {
        'Content-Type': 'application/json',
      },
    });
  }
}
