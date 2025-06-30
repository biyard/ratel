import { NextResponse } from 'next/server';
import type { NextRequest } from 'next/server';
import { v4 as uuidv4 } from 'uuid';

export function middleware(req: NextRequest) {
  let sessionId = req.cookies.get('nx_session_id')?.value;
  const exists = sessionId !== undefined;

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

  return res;
}

export const config = {
  matcher: '/((?!_next|favicon.ico).*)',
};
