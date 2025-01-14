import { NextRequest } from 'next/server';

const corsHeaders = {
  'Access-Control-Allow-Origin': '*',
  'Access-Control-Allow-Methods': 'GET, POST, PUT, DELETE, OPTIONS',
  'Access-Control-Allow-Headers': '*',
};

export async function GET(
  request: NextRequest,
  { params }: { params: { path: string[] } }
) {
  return handleProxyRequest(request, params.path.join('/'));
}

export async function POST(
  request: NextRequest,
  { params }: { params: { path: string[] } }
) {
  return handleProxyRequest(request, params.path.join('/'));
}

export async function OPTIONS() {
  return new Response(null, {
    headers: corsHeaders,
    status: 204,
  });
}

async function handleProxyRequest(request: NextRequest, path: string) {
  try {
    if (!path) {
      return Response.json({ error: 'Invalid proxy URL format' }, { status: 400 });
    }

    const url = new URL(request.url);
    const targetURL = `https://${path}${url.search}`;

    // Forward the request to the target URL
    const response = await fetch(targetURL, {
      method: request.method,
      headers: {
        ...Object.fromEntries(request.headers),
        host: new URL(targetURL).host,
      },
      body: ['GET', 'HEAD'].includes(request.method) ? null : await request.arrayBuffer(),
    });

    // Create response with the original response and CORS headers
    const responseHeaders = new Headers(response.headers);
    Object.entries(corsHeaders).forEach(([key, value]) => {
      responseHeaders.set(key, value);
    });

    return new Response(response.body, {
      status: response.status,
      statusText: response.statusText,
      headers: responseHeaders,
    });
  } catch (error) {
    console.error('Error in git-proxy:', error);
    return Response.json({ error: 'Internal Server Error' }, { status: 500 });
  }
}
