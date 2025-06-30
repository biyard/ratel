// import News from './_components/News';
// import { CreatePost } from './_components/create-post';
// import { useApiCall } from '@/lib/api/use-send';
// import { ratelApi } from '@/lib/api/ratel_api';
// import {
//   UrlType,
//   writePostRequest,
// } from '@/lib/api/models/feeds/write-post-request';
import { Metadata } from 'next';
import TeamHome from './page.client';
import { client } from '@/lib/apollo';
import { ratelApi } from '@/lib/api/ratel_api';
import { Feed, FeedStatus } from '@/lib/api/models/feeds';
import { callByServer } from '@/lib/api/call-by-server';
import { QueryResponse } from '@/lib/api/models/common';

export const metadata: Metadata = {
  title: 'Ratel',
  description:
    'The first platform connecting South Korea’s citizens with lawmakers to drive institutional reform for the crypto industry.Are you with us ?',
  icons: {
    icon: 'https://ratel.foundation/favicon.ico',
    apple: 'https://ratel.foundation/favicon.ico',
  },
  openGraph: {
    title: 'Ratel',
    description:
      'The first platform connecting South Korea’s citizens with lawmakers to drive institutional reform for the crypto industry.Are you with us ?',
    url: 'https://ratel.foundation',
    siteName: 'Ratel',
    images: [
      {
        url: 'https://metadata.ratel.foundation/logos/logo-symbol.png',
      },
    ],
    locale: 'en_US',
  },
  twitter: {
    card: 'summary_large_image',
    title: 'Ratel',
    description:
      'The first platform connecting South Korea’s citizens with lawmakers to drive institutional reform for the crypto industry.Are you with us ?',
    images: ['https://metadata.ratel.foundation/logos/logo-symbol.png'],
  },
};

type Props = {
  params: Promise<{ username: string }>;
};

export default async function Page({ params }: Props) {
  const { username } = await params;
  const {
    data: { users },
  } = await client.query(ratelApi.graphql.getTeamByTeamname(username));

  if (users.length === 0) {
    // FIXME: fix this to use not-found.tsx
    return <div className="text-center">Team not found</div>;
  }

  const userId = users[0].id;
  const { get } = callByServer();

  const posts: QueryResponse<Feed> = await get(
    ratelApi.feeds.getPostsByUserId(userId, 1, 20, FeedStatus.Published),
  );

  return <TeamHome userId={userId} posts={posts} />;
}
