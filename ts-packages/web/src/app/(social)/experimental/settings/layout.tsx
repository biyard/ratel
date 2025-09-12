import { Suspense } from 'react';
import Loading from '@/app/loading';
import Provider from '../../providers';

export default async function SettingLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <Provider>
      <div className="flex flex-col w-full min-h-screen justify-between max-w-desktop mx-auto text-white pt-3 gap-5 max-tablet:px-5 mb-8">
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
    </Provider>
  );
}
