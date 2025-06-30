/* eslint-disable @typescript-eslint/no-explicit-any */

import { config } from '@/config';
import { logger } from '../logger';
import { getCookieContext } from '@/app/_providers/CookieProvider';

export interface ApiCallFns {
  get: (path: string) => Promise<any>;
  post: (path: string, body?: any) => Promise<any>;
}

export function callByServer(): ApiCallFns {
  return {
    get: async (path: string): Promise<any> => {
      const cookie = await getCookieContext();
      const apiBaseUrl: string = config.api_url;
      const token = cookie?.token;
      const token_type = 'Bearer';

      const headers: any = { 'Content-Type': 'application/json' };
      if (token) headers['Authorization'] = `${token_type} ${token}`;

      const response = await fetch(`${apiBaseUrl}${path}`, {
        method: 'GET',
        headers,
      });

      if (!response.ok) {
        return null;
      }

      return response.json();
    },
    post: async (path: string, body?: any): Promise<any> => {
      const cookie = await getCookieContext();

      const apiBaseUrl: string = config.api_url;
      const token = cookie?.token;
      const token_type = 'Bearer';

      const headers: any = { 'Content-Type': 'application/json' };
      if (token) headers['Authorization'] = `${token_type} ${token}`;

      const response = await fetch(`${apiBaseUrl}${path}`, {
        method: 'POST',
        headers,
        body: body ? JSON.stringify(body) : undefined,
      });

      if (!response.ok) {
        const errorData = await response.json().catch(() => ({
          message: 'Failed to fetch and parse error',
        }));
        logger.error('Failed to fetch and parse error ', errorData?.message);
        return null;
      }

      return response.json();
    },
  };
}
