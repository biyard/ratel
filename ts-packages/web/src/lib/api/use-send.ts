/* eslint-disable @typescript-eslint/no-explicit-any */

'use client';

import { config } from '@/config';
import { logger } from '../logger';

export interface ApiCallFns {
  get: (path: string) => Promise<any>;
  post: (path: string, body?: any) => Promise<any>;
}

/**
 * @deprecated Use `Call('GET', path)` from '@/lib/api/ratel/call' instead.
 */

export function useApiCall(): ApiCallFns {
  return {
    get: async (path: string): Promise<any> => {
      const apiBaseUrl: string = config.api_url;
      const headers: any = { 'Content-Type': 'application/json' };

      const response = await fetch(`${apiBaseUrl}${path}`, {
        method: 'GET',
        headers,
        credentials: 'include',
      });

      if (!response.ok) {
        return null;
      }

      return response.json();
    },
    post: async (path: string, body?: any): Promise<any> => {
      const apiBaseUrl: string = config.api_url;

      const headers: any = { 'Content-Type': 'application/json' };

      const response = await fetch(`${apiBaseUrl}${path}`, {
        method: 'POST',
        headers,
        credentials: 'include',
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
