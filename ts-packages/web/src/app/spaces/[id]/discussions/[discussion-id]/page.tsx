import React from 'react';

import type { Metadata } from 'next';
import { ratelApi } from '@/lib/api/ratel_api';
import { Space } from '@/lib/api/models/spaces';
import { Feed } from '@/lib/api/models/feeds';
import { logger } from '@/lib/logger';
import { config } from '@/config';
import striptags from 'striptags';
import DiscussionByIdPage from './page.client';

type Props = {
  params: Promise<{ id: number; discussion_id: number }>;
};

export async function generateMetadata({ params }: Props): Promise<Metadata> {
  const id = (await params).id;
  const space: Space = await fetch(
    `${config.api_url}${ratelApi.spaces.getSpaceBySpaceId(id)}`,
  ).then((res) => res.json());

  const feed: Feed = await fetch(
    `${config.api_url}${ratelApi.feeds.getFeedsByFeedId(space.feed_id)}`,
  ).then((res) => res.json());

  const description = striptags(space.html_contents);

  return {
    title: space.title ?? feed.title!,
    description,
    openGraph: {
      title: space.title ?? feed.title!,
      description,
      images: [
        {
          url: feed.url!,
        },
      ],
    },
  };
}

export default async function DiscussionPage({ params }: Props) {
  logger.debug('DiscussionPage params', await params);

  return <DiscussionByIdPage />;
}
