import { getDeliberationSpaceById } from '@/lib/api/ratel_api.server';
import { Metadata } from 'next';
import React, { Suspense } from 'react';
import striptags from 'striptags';
import Loading from '@/app/loading';
import Provider from './providers';

export async function generateMetadata({
  params,
}: {
  params: Promise<{ id: string }>;
}): Promise<Metadata> {
  const { id } = await params;

  const { data } = await getDeliberationSpaceById(id);

  const title = data?.title ?? undefined;
  const description = data ? striptags(data.summary.html_contents) : undefined;
  //   let images = undefined;
  //   if (data) {
  //     const { data: feed } = await getFeedById(data?.feed_id);
  //     if (feed && feed.url) {
  //       images = [{ url: feed.url }];
  //     }
  //   }

  return {
    title,
    description,
    openGraph: {
      title,
      description,
      //   images,
    },
  };
}

export default async function Layout({
  children,
  params,
}: {
  children: React.ReactNode;
  params: Promise<{ id: string }>;
}) {
  const { id } = await params;
  return (
    <Provider spaceId={id}>
      <div className="flex flex-col w-full min-h-[100svh] justify-between max-w-desktop mx-auto text-white pt-3 gap-5 max-tablet:px-5 mb-8 max-desktop:max-w-screen">
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
