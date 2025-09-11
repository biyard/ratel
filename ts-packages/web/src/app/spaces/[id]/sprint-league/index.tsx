import { getOption as getSpaceByIdOption } from '@/hooks/use-space-by-id';
import { getOption as getFeedByIdOption } from '@/hooks/use-feed-by-id';
import { getQueryClient } from '@/providers/getQueryClient';
import Header from '../_components/common-header';
import { SSRHydration } from '@/lib/query-utils';
import { SprintLeagueEditor } from './editor';
import SpaceSideMenu from './_components/side-menu';
import Base from './_components/konva';
export default async function SprintLeaguePage({
  space_id,
}: {
  space_id: number;
}) {
  const queryClient = getQueryClient();
  const { feed_id } = await queryClient.fetchQuery(
    getSpaceByIdOption(space_id),
  );

  await Promise.allSettled([
    queryClient.prefetchQuery(getFeedByIdOption(feed_id)),
  ]);

  return (
    <SSRHydration queryClient={queryClient}>
      <div className="flex flex-row w-full gap-5">
        <div className="flex flex-col w-full min-h-full gap-6.25">
          <Header />
          <Base />
          {/* <SprintLeagueEditor spaceId={space_id} /> */}
        </div>
        <SpaceSideMenu spaceId={space_id} />
      </div>
    </SSRHydration>
  );
}
