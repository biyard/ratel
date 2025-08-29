import { config } from '@/config';
import { NextRequest, NextResponse } from 'next/server';

export async function GET(request: NextRequest) {
  return new NextResponse({
    version: config.version,
  });

}
