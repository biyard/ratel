'use client';
import React from 'react';
import { DotLottieReact } from '@lottiefiles/dotlottie-react';
import { cn } from '@/lib/utils';

export default function Loading({ className }: { className?: string }) {
  return (
    <div className="w-screen h-screen fixed top-0 left-0 z-999 flex items-center justify-center bg-popup-background backdrop-blur-[10px]">
      <DotLottieReact
        className={cn('max-w-50 max-h-50', className)}
        src="/animations/loading.json"
        loop
        autoplay
      />
    </div>
  );
}
