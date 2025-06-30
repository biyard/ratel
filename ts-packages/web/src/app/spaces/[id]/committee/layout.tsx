import Loading from '@/app/loading';
import React, { Suspense } from 'react';
import SpaceSideMenu from '../_components/space_side_menu';
import CreateCommentBox from '../_components/create_comment_box';
import SpaceComments from './_components/space-comments';
import Provider from './providers';

interface LayoutProps {
  children: React.ReactNode;
  spaceId: number;
}

export default async function CommitteeSpaceLayout({
  children,
  spaceId,
}: LayoutProps) {
  return (
    <Provider spaceId={spaceId}>
      <div className="flex flex-col w-full min-h-screen justify-between max-w-desktop mx-auto text-white pt-3 gap-5 max-tablet:px-5 mb-8">
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
          <SpaceSideMenu spaceId={spaceId} />
        </div>
        <SpaceComments spaceId={spaceId} />
        <CreateCommentBox spaceId={spaceId} />
      </div>
    </Provider>
  );
}
