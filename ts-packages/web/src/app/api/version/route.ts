import { config } from '@/config';
import { NextResponse } from 'next/server';

export async function GET() {
  return NextResponse.json({
    version: config.version,
  });
}
