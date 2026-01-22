import { Suspense } from 'react';
import { Outlet, useParams } from 'react-router';
import Loading from '@/app/loading';
import TeamSidemenu from './_components/team-sidemenu';

export default function TeamLayout() {
  const { username } = useParams<{ username: string }>();
  // FIXME:
  if (!username) {
    return <div>Team not found</div>;
  }

  return (
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
          <Outlet />
        </Suspense>
      </div>
    </div>
  );
}
