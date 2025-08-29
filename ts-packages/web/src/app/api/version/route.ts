import { config } from '@/config';
import { NextRequest, NextResponse } from 'next/server';

export async function GET() {
  return new NextResponse({
    version: config.version,
  });

}
