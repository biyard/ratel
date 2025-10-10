import { Outlet } from 'react-router';

export default function SpaceLayout() {
  return (
    <div className="flex flex-col w-full min-h-[100svh] justify-between max-w-desktop mx-auto text-white pt-3 gap-5 max-tablet:px-5 mb-8 max-desktop:max-w-screen">
      <Outlet />
    </div>
  );
}
