import { Suspense } from 'react';
import { Outlet, useParams } from 'react-router';
import Loading from '@/app/loading';
import { logger } from '@/lib/logger';
import TeamSidemenu from './_components/team-sidemenu';
import Provider from './providers';
import {
  CreatePost,
  PostEditorProvider,
} from '@/app/(social)/_components/post-editor';
import {
  CreateRePost,
  RePostDraftProvider,
} from '@/app/(social)/_components/create-repost';

export default function TeamLayout() {
  const { username } = useParams<{ username: string }>();
  console.log('TeamLayout: useParams result:', { username });
  console.log('TeamLayout: window.location.pathname:', window.location.pathname);
  logger.debug('TeamLayout: username', username);
  
  if (!username) {
    return <div>Team not found</div>;
  }

  return (
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
            <PostEditorProvider>
              <RePostDraftProvider>
                <Outlet />

                <div className="fixed bottom-0 left-0 right-0 z-10 flex flex-row items-center justify-center">
                  <div className="max-w-desktop w-full">
                    <CreatePost />
                    <CreateRePost />
                  </div>
                </div>
              </RePostDraftProvider>
            </PostEditorProvider>
          </Suspense>
        </div>
      </div>
    </Provider>
  );
}
