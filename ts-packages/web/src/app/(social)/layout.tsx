import UserSidemenu from './_components/user-sidemenu';

import { Outlet } from 'react-router';

export default function SocialLayout() {
  return (
    <div className="flex min-h-screen max-mobile:gap-0 gap-5 justify-between max-w-desktop mx-auto text-white py-3 max-tablet:px-2.5 overflow-x-hidden">
      <UserSidemenu />
      <div className="flex grow">
        <Outlet />
        <div className="fixed bottom-0 left-0 right-0 z-10 flex flex-row items-center justify-center"></div>
      </div>
    </div>
  );
}
