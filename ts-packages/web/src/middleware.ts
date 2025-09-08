import { NextResponse } from 'next/server';
import type { NextRequest } from 'next/server';
import { v4 as uuidv4 } from 'uuid';
import { logger } from './lib/logger';

export function middleware(req: NextRequest) {
  const start = Date.now();
  logger.info(`${req.method} ${req.url}`);
  let sessionId = req.cookies.get('nx_session_id')?.value;
  const exists = sessionId !== undefined;

  if (req.method === 'OPTIONS') {
    const latency = Date.now() - start;
    logger.info(`OPTIONS ${req.url} - ${latency}ms`);

    return new NextResponse(null, {
      status: 200,
      headers: {
        'Access-Control-Allow-Origin': '*',
        'Access-Control-Allow-Methods': '*',
        'Access-Control-Allow-Headers': '*',
      },
    });
  }

  if (!sessionId) {
    sessionId = uuidv4();
    req.cookies.set('nx_session_id', sessionId);
  }

  const res = NextResponse.next({
    request: req,
  });

  if (!exists) {
    res.cookies.set('nx_session_id', sessionId, {
      httpOnly: true,
      path: '/',
      sameSite: 'lax',
    });
  }
  const latency = Date.now() - start;
  logger.info(`${req.method} ${req.url} - ${latency}ms`);

  return res;
}

export const config = {
  matcher: '/((?!_next|favicon.ico).*)',
};
