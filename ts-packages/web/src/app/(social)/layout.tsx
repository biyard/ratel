import UserSidemenu from './_components/user-sidemenu';
import { CreatePost, PostEditorProvider } from './_components/post-editor';
import { CreateRePost, RePostDraftProvider } from './_components/create-repost';
import { Outlet } from 'react-router';

export default function SocialLayout() {
  return (
    <div className="flex overflow-x-hidden gap-5 justify-between py-3 mx-auto min-h-screen text-white max-w-desktop max-tablet:px-2.5">
      <UserSidemenu />
      <div className="flex grow">
        <PostEditorProvider>
          <RePostDraftProvider>
            <Outlet />
            <div className="flex fixed right-0 bottom-0 left-0 z-10 flex-row justify-center items-center">
              <div className="w-full max-w-desktop">
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
