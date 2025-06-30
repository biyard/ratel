'use client';
import Loading from '@/app/loading';
import React, { Suspense } from 'react';

interface LayoutProps {
  children: React.ReactNode;
  params: Promise<{
    id: string;
    discussion_id: string;
  }>;
}

export default function DiscussionLayout({ children }: LayoutProps) {
  return (
    <div className="flex flex-col w-full min-h-screen justify-between text-white">
      <div className="flex flex-row w-full gap-5">
        <div className="flex-1 flex w-full">
          <Suspense
            fallback={
              <div className="fixed top-0 left-0 w-full h-full flex items-center justify-center">
                <Loading />
              </div>
            }
          >
            {children}
          </Suspense>
        </div>
      </div>
    </div>
  );
}
