import { config } from '@/config';
import { ratelApi } from '@/lib/api/ratel_api';
import { logger } from '@/lib/logger';
import { NextRequest, NextResponse } from 'next/server';

export async function GET(request: NextRequest) {
  logger.debug('[GET /api/login] incoming');
  const password = request.nextUrl.searchParams.get('password');
  const email = request.nextUrl.searchParams.get('email');

  const apiBaseUrl: string = config.api_url;

  let targetUrl = `${apiBaseUrl}${ratelApi.users.login()}`;
  if (email && password && password !== '') {
    const path = ratelApi.users.loginWithPassword(email, password);
    targetUrl = `${apiBaseUrl}${path}`;
  }

  logger.debug('request headers', request.headers);
  logger.debug('targetUrl', targetUrl);
  const res = await fetch(targetUrl, {
    method: 'GET',
    headers: {
      'Content-Type': 'application/json',
      Authorization: request.headers.get('authorization') || '',
    },
  });

  const protocol = request.headers.get('x-forwarded-proto') || 'http';
  const host = request.headers.get('host') || 'localhost';
  const port = request.headers.get('x-forwarded-port') || '8080';
  let origin = `${protocol}://${host}`;

  if (port && port !== '80' && port !== '443') {
    origin += `:${port}`;
  }

  logger.debug('response header', res.headers);
  const setCookies: string[] = res.headers.getSetCookie();
  logger.debug('raw set-cookie headers:', setCookies);

  const idCookie = setCookies.find((c) => c.startsWith('id='));
  const authCookie = setCookies.find((c) => c.startsWith('auth_token='));

  const commonSuffix =
    protocol === 'https'
      ? `Path=/; HttpOnly; Secure; SameSite=None; Domain=.${host}; Max-Age=2586226`
      : 'Path=/; SameSite=Lax; Max-Age=2586226';

  const cookies: string[] = [];

  if (idCookie) {
    cookies.push(idCookie.split(';')[0] + '; ' + commonSuffix);
  }

  if (authCookie) {
    cookies.push(authCookie.split(';')[0] + '; ' + commonSuffix);
  }

  return new NextResponse(res.body, {
    status: res.status,
    headers: {
      'Access-Control-Allow-Origin': origin,
      'Access-Control-Allow-Methods': 'GET',
      'Access-Control-Allow-Headers': 'Content-Type, Authorization',
      'Access-Control-Allow-Credentials': 'true',
      'Set-Cookie': cookies.join(', '),
    },
  });
}

export async function OPTIONS() {
  return new NextResponse(null, {
    status: 200,
    headers: {
      'Access-Control-Allow-Origin': '*',
      'Access-Control-Allow-Methods': '*',
      'Access-Control-Allow-Headers': '*',
    },
  });
}
