import { config } from '@/config';
import { logger } from '@/lib/logger';

export type HttpMethod = 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH';

export async function call<T, R>(
  method: HttpMethod,
  path: string,
  body?: T,
): Promise<R> {
  const apiBaseUrl: string = config.api_url;

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const headers: any = { 'Content-Type': 'application/json' };

  const response = await fetch(`${apiBaseUrl}${path}`, {
    method,
    headers,
    credentials: 'include',
    body: body ? JSON.stringify(body) : undefined,
  });

  if (!response.ok) {
    const errorData = await response
      .text()
      .catch((e) => `Failed to fetch and parse error ${e}`);
    logger.error('Failed to fetch and parse error ', errorData);

    throw new RatelSdkError(errorData);
  }

  return response.json();
}

export class RatelSdkError extends Error {
  readonly name: string;
  readonly message: string;

  constructor(message: string) {
    super(message);
    this.name = 'RatelSdkError';
    this.message = message;
  }
}
