import { Suspense } from 'react';
import UserSidemenu from './_components/user-sidemenu';
import Loading from '../loading';
import { CreatePost, PostDraftProvider } from './_components/create-post';
import { CreateRePost, RePostDraftProvider } from './_components/create-repost3';
import Provider from './providers';

export default async function SocialLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <Provider>
      <div className="flex min-h-screen gap-5 justify-between max-w-desktop mx-auto text-white py-3 max-tablet:px-2.5 overflow-x-hidden">
        <UserSidemenu />
        <div className="flex grow">
          <Suspense
            fallback={
              <div className="fixed top-0 left-0 w-full h-full flex items-center justify-center">
                <Loading />
              </div>
            }
          >
            <PostDraftProvider>
              <RePostDraftProvider>
                {children}

                <div className="fixed bottom-0 left-0 right-0 z-10 flex flex-row items-center justify-center">
                  <div className="max-w-desktop w-full">
                    <CreatePost />
                    <CreateRePost/>
                  </div>
                </div>
              </RePostDraftProvider>
            </PostDraftProvider>
          </Suspense>
        </div>
      </div>
    </Provider>
  );
}
