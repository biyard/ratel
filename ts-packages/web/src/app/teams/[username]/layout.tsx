import { Suspense } from 'react';
import Loading from '@/app/loading';
import { logger } from '@/lib/logger';
import TeamSidemenu from './_components/team-sidemenu';
import { CreatePost, PostDraftProvider } from './_components/create-post';
import { dehydrate, HydrationBoundary } from '@tanstack/react-query';
import { getServerQueryClient } from '@/lib/query-utils.server';
import Provider from './providers';

export interface TeamLayoutProps {
  params: Promise<{ username: string }>;
}

export default async function TeamLayout({
  children,
  params,
}: Readonly<{
  children: React.ReactNode;
}> &
  TeamLayoutProps) {
  const queryClient = await getServerQueryClient();
  const dehydratedState = dehydrate(queryClient);
  const { username } = await params;
  logger.debug('TeamLayout: username', username);

  return (
    <HydrationBoundary state={dehydratedState}>
      <Provider username={username}>
        <div className="flex min-h-screen justify-between max-w-desktop mx-auto text-white pt-3 gap-[20px]">
          <TeamSidemenu username={username} />
          <div className="flex-1 flex">
            <Suspense
              fallback={
                <div className="w-full h-full flex items-center justify-center">
                  <Loading />
                </div>
              }
            >
              <PostDraftProvider username={username}>
                {children}

                <div className="fixed bottom-0 left-0 right-0 z-10 flex flex-row items-center justify-center">
                  <div className="max-w-desktop w-full">
                    <CreatePost />
                  </div>
                </div>
              </PostDraftProvider>
            </Suspense>
          </div>
        </div>
      </Provider>
    </HydrationBoundary>
  );
}
