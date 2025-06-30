import { logger } from '@/lib/logger';
import { NextRequest, NextResponse } from 'next/server';

//FIXME: This logic needs to be implemented on the backend. And just call the logout endpoint like proxy.
export async function POST(request: NextRequest) {
  const host = request.headers.get('host') || 'localhost';

  logger.debug('host', host);

  const response = NextResponse.json(
    { message: 'Logout successful' },
    { status: 200 },
  );

  if (host.includes('localhost')) {
    response.cookies.set('id', '', {
      maxAge: 0,
      path: '/',
      sameSite: 'lax',
      domain: 'localhost',
    });
    response.cookies.set('auth_token', '', {
      maxAge: 0,
      path: '/',
      sameSite: 'lax',
      domain: 'localhost',
    });
  } else {
    response.cookies.set('id', '', {
      maxAge: 0,
      path: '/',
      sameSite: 'none',
      domain: `.${host}`,
      secure: true,
      httpOnly: true,
    });
    response.cookies.set('auth_token', '', {
      maxAge: 0,
      path: '/',
      sameSite: 'none',
      domain: `.${host}`,
      secure: true,
      httpOnly: true,
    });
  }

  return response;
}

export async function OPTIONS() {
  return new NextResponse(null, {
    status: 200,
    headers: {
      'Access-Control-Allow-Origin': '*',
      'Access-Control-Allow-Methods': 'POST, OPTIONS',
      'Access-Control-Allow-Headers': 'Content-Type, Authorization',
    },
  });
}
