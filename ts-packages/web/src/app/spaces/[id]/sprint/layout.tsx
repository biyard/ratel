import Loading from '@/app/loading';
import React, { Suspense } from 'react';
import Provider from './providers';

interface LayoutProps {
  children: React.ReactNode;
  spaceId: number;
}

export default async function SprintLeagueLayout({
  children,
  spaceId,
}: LayoutProps) {
  console.log('Hello');
  return (
    <Provider spaceId={spaceId}>
      <div className="fixed inset-0 w-screen h-screen z-[9999] bg-neutral-800">
        <div className="flex flex-row w-full h-full gap-5">
          <div className="flex-1 flex">
            <Suspense
              fallback={
                <div className="fixed inset-0 flex items-center justify-center">
                  <Loading />
                </div>
              }
            >
              {children}
            </Suspense>
          </div>
        </div>
      </div>
    </Provider>
  );
}
