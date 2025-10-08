import UserSidemenu from './_components/user-sidemenu';
import { CreatePost, PostEditorProvider } from './_components/post-editor';
import { CreateRePost, RePostDraftProvider } from './_components/create-repost';
import { Outlet } from 'react-router';

export default function SocialLayout() {
  return (
    <div className="flex min-h-screen gap-5 justify-between max-w-desktop mx-auto text-white py-3 max-tablet:px-2.5 overflow-x-hidden">
      <UserSidemenu />
      <div className="flex grow">
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
      </div>
    </div>
  );
}
