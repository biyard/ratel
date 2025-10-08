import { logger } from '../logger';

export type FetchResponse<T = unknown> = {
  data: T;
  status: number;
  statusText: string;
  headers: Headers;
  ok: boolean;
};

export async function apiFetch<T = unknown>(
  url: string,
  options?: RequestInit & {
    ignoreError?: boolean;
    isServer?: boolean;
  },
): Promise<FetchResponse<T | null>> {
  const requestHeaders = new Headers(options?.headers);
  const method = options?.method || 'GET';
  const body = options?.body;

  if (!requestHeaders.has('Content-Type') && body && typeof body === 'string') {
    requestHeaders.set('Content-Type', 'application/json');
  }

  try {
    const response = await fetch(url, {
      ...options,
      method,
      headers: requestHeaders,
      credentials: 'include',
      cache: options?.cache || 'no-store',
      ...(method === 'GET' || method === 'HEAD' ? {} : { body }),
    });

    if (!response.ok) {
      const errorBody = await response.text();
      logger.error(
        `Error fetching ${url}: ${response.status} ${response.statusText} ${errorBody}`,
      );
      if (options?.ignoreError) {
        return {
          data: null,
          status: response.status,
          statusText: response.statusText,
          headers: response.headers,
          ok: false,
        };
      }
      throw new Error(`API Error: ${response.status} ${response.statusText}`);
    }
    let data: T | null = null;
    if (response.status !== 204) {
      const contentType = response.headers.get('Content-Type') ?? '';
      data = contentType.includes('application/json')
        ? ((await response.json()) as T)
        : ((await response.text()) as unknown as T);
    }
    return {
      data,
      status: response.status,
      statusText: response.statusText,
      headers: response.headers,
      ok: true,
    };
  } catch (error) {
    logger.error(`Error during server-side fetch to ${url}:`, error);
    if (options?.ignoreError) {
      return {
        data: null,
        status: 500,
        statusText:
          error instanceof Error ? error.message : 'Unknown server error',
        headers: new Headers(),
        ok: false,
      };
    }
    throw error;
  }
}
