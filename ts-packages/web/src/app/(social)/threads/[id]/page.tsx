import { SSRHydration } from '@/lib/query-utils';
import { getQueryClient } from '@/providers/getQueryClient';
import Header from './_components/header';
import ThreadPost from './_components/thread';
import ThreadComment from './_components/comment';

import { requestFeedByID, setInitialFeedByID } from '../../_hooks/feed';
import { Metadata } from 'next';
import striptags from 'striptags';
import { cache, Suspense } from 'react';
import { notFound } from 'next/navigation';
import { logger } from '@/lib/logger';

interface Props {
  params: Promise<{ id: string }>;
}

const getCachedFeedByID = cache(async (id: number) => {
  return requestFeedByID(id);
});

export async function generateMetadata({ params }: Props): Promise<Metadata> {
  const { id } = await params;
  const feed_id = parseInt(id, 10);

  let title = 'Ratel Thread';
  let description = 'Ratel Thread';
  let image = '';
  try {
    const { data: feed } = await getCachedFeedByID(feed_id);
    if (feed) {
      title = feed.title || title;
      description = striptags(feed.html_contents);
      image = feed.url || '';
    }
  } catch (error) {
    logger.error(`Failed to generate metadata for feed ${feed_id}:`, error);
  }

  return {
    title,
    description,
    openGraph: {
      title,
      description,
      images: [
        {
          url: image,
        },
      ],
    },
  };
}

export default async function Page({ params }: Props) {
  const { id } = await params;
  const feed_id = parseInt(id, 10);
  const queryClient = getQueryClient();
  const { data } = await getCachedFeedByID(feed_id);
  if (!data) {
    notFound();
  }

  setInitialFeedByID(queryClient, feed_id, data);

  return (
    <SSRHydration queryClient={queryClient}>
      <Suspense fallback={<div>Loading...</div>}>
        <div className="flex flex-col gap-6 w-full">
          <Header post_id={feed_id} />
          <ThreadPost post_id={feed_id} />
          <ThreadComment post_id={feed_id} />
        </div>
      </Suspense>
    </SSRHydration>
  );
}
