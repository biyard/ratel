import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"
import { sha256 } from "ethers"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export function sha3(str: string): string {
  const encoder = new TextEncoder();
  const data = encoder.encode(str);
  const hashed = sha256(data);
  return hashed;
}
