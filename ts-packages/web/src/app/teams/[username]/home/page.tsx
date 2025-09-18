import { Metadata } from 'next';
import TeamHome from './page.client';
import { ratelApi } from '@/lib/api/ratel_api';
import { prefetchInfiniteFeeds } from '@/hooks/feeds/use-feeds-infinite-query';
import { FeedStatus } from '@/lib/api/models/feeds';
import { apiFetch } from '@/lib/api/apiFetch';
import { config } from '@/config';

//FIXME: add Metadata
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
  const userResp = await apiFetch<{ id: number } | null>(
    `${config.api_url}${ratelApi.users.getUserByUsername(username)}`,
    { ignoreError: true, cache: 'no-store' },
  );

  if (!userResp?.data?.id) {
    // FIXME: fix this to use not-found.tsx
    return <div className="text-center">Team not found</div>;
  }
  console.log('user', userResp.data);
  await Promise.allSettled([
    prefetchInfiniteFeeds(userResp.data.id, FeedStatus.Published),
  ]);

  return <TeamHome teamId={userResp.data.id} />;
}
