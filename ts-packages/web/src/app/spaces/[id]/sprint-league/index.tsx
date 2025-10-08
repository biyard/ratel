import { getOption as getSpaceByIdOption } from '@/hooks/use-space-by-id';
import { getOption as getFeedByIdOption } from '@/hooks/feeds/use-feed-by-id';
import { getQueryClient } from '@/providers/getQueryClient';
import Header from '../_components/common-header';
import { SSRHydration } from '@/lib/query-utils';
import { SprintLeagueEditor } from './editor';
import SpaceSideMenu from './_components/side-menu';
export default async function SprintLeaguePage({
  space_id,
}: {
  space_id: number;
}) {
  const queryClient = getQueryClient();
  const { feed_id } = await queryClient.fetchQuery(
    getSpaceByIdOption(space_id),
  );

  // TODO: Update space API to use string feed_id in v3
  await Promise.allSettled([
    queryClient.prefetchQuery(getFeedByIdOption(feed_id.toString())),
  ]);

  return (
    <SSRHydration queryClient={queryClient}>
      <div className="flex flex-row w-full gap-5">
        <div className="flex flex-col w-full min-h-full gap-6.25">
          <Header />
          <SprintLeagueEditor spaceId={space_id} />
        </div>
        <SpaceSideMenu spaceId={space_id} />
      </div>
    </SSRHydration>
  );
}
