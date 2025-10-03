import { config } from '@/config';
import { logger } from '@/lib/logger';

export type HttpMethod = 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH';

export async function call<T, R>(
  method: HttpMethod,
  path: string,
  body?: T,
): Promise<R> {
  const apiBaseUrl: string = config.api_url;

  let response;
  if (body) {
    response = await fetch(`${apiBaseUrl}${path}`, {
      method,
      headers: { 'Content-Type': 'application/json' },
      credentials: 'include',
      body: body ? JSON.stringify(body) : undefined,
    });
  } else {
    response = await fetch(`${apiBaseUrl}${path}`, {
      method,
      credentials: 'include',
    });
  }

  if (!response.ok) {
    const errorData = await response
      .text()
      .catch((e) => `Failed to fetch and parse error ${e}`);
    logger.error('Failed to fetch and parse error ', errorData);

    throw new RatelSdkError(errorData);
  }

  const json_body: R = await response.json();
  logger.debug(
    `API Response Body(${method} ${apiBaseUrl}${path}): `,
    json_body,
  );

  return json_body;
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
