import { getOption as getSpaceByIdOption } from '@/hooks/use-space-by-id';
import { getOption as getFeedByIdOption } from '@/hooks/use-feed-by-id';
import { getQueryClient } from '@/providers/getQueryClient';
import { SSRHydration } from '@/lib/query-utils';
import Header from '../_components/common-header';
import SideMenu from './_components/side-menu';
import Initial from './_components/initial';
import { getOption as getDagitByIdOption } from '@/hooks/use-dagit';
import MainTab from './_components/tab';

export default async function DAgitPage({ spaceId }: { spaceId: number }) {
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
          <MainTab spaceId={spaceId} />
        </div>
        <SideMenu spaceId={spaceId} />
      </div>
    </SSRHydration>
  );
}
