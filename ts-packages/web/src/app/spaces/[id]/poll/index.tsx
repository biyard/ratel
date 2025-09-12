import { getOption as getSpaceByIdOption } from '@/hooks/use-space-by-id';
import { getOption as getFeedByIdOption } from '@/hooks/feeds/use-feed-by-id';
import { getQueryClient } from '@/providers/getQueryClient';
import { SSRHydration } from '@/lib/query-utils';
import Header from '../_components/common-header';
import { getOption as getDagitByIdOption } from '@/hooks/use-dagit';
import Initial from './_components/initial';
import MainTab from './_components/main';
import Content from './_components/content';
import SpaceSideMenu from './_components/side-menu';

export default async function PollPage({ spaceId }: { spaceId: number }) {
  const queryClient = getQueryClient();
  const { feed_id } = await queryClient.fetchQuery(getSpaceByIdOption(spaceId));

  await Promise.allSettled([
    queryClient.prefetchQuery(getFeedByIdOption(feed_id)),
    queryClient.prefetchQuery(getDagitByIdOption(spaceId)),
  ]);

  return (
    <SSRHydration queryClient={queryClient}>
      <Initial spaceId={spaceId} />
      <div className="flex flex-row w-full gap-5">
        <div className="flex flex-col w-full min-h-full gap-6.25">
          <Header />
          <Content spaceId={spaceId} />
          <MainTab spaceId={spaceId} />
        </div>
        <SpaceSideMenu spaceId={spaceId} />
      </div>
    </SSRHydration>
  );
}
