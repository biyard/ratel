import { clsx, type ClassValue } from 'clsx';
import { sha256 } from 'ethers';
import { twMerge } from 'tailwind-merge';
import { logger } from './logger';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function sha3(str: string): string {
  const encoder = new TextEncoder();
  const data = encoder.encode(str);
  const hashed = sha256(data);
  logger.debug('SHA3 Hash:', hashed);

  return hashed;
}
