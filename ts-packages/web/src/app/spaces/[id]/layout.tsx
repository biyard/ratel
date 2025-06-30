import { getFeedById, getSpaceById } from '@/lib/api/ratel_api.server';
import { Metadata } from 'next';
import React, { Suspense } from 'react';
import Provider from './providers';
import striptags from 'striptags';
import Loading from '@/app/loading';
import { SpaceType } from '@/lib/api/models/spaces';
import CommitteeSpaceLayout from './committee/layout';
import CommitteeSpacePage from './committee/page.client';

export async function generateMetadata({
  params,
}: {
  params: Promise<{ id: number }>;
}): Promise<Metadata> {
  const { id } = await params;

  const { data } = await getSpaceById(id);

  const title = data?.title ?? undefined;
  const description = data ? striptags(data.html_contents) : undefined;
  let images = undefined;
  if (data) {
    const { data: feed } = await getFeedById(data?.feed_id);
    if (feed && feed.url) {
      images = [{ url: feed.url }];
    }
  }

  return {
    title,
    description,
    openGraph: {
      title,
      description,
      images,
    },
  };
}

export default async function Layout({
  children,
  params,
}: {
  children: React.ReactNode;
  params: Promise<{ id: number }>;
}) {
  const { id } = await params;
  const { data } = await getSpaceById(id);
  const spaceType = data?.space_type;

  if (spaceType === SpaceType.Committee) {
    return (
      <CommitteeSpaceLayout spaceId={id}>
        <CommitteeSpacePage />
      </CommitteeSpaceLayout>
    );
  }

  return (
    <Provider spaceId={id}>
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
        </div>
      </div>
    </Provider>
  );
}
