import { getOption as getSpaceByIdOption } from '@/hooks/use-space-by-id';
import { getOption as getFeedByIdOption } from '@/hooks/use-feed-by-id';
import { getQueryClient } from '@/providers/getQueryClient';
import Header from '../_components/common-header';
import { SSRHydration } from '@/lib/query-utils';
import { SprintLeagueEditor } from './editor';
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
      <div className="flex flex-col w-full min-h-full gap-6.25">
        <Header />
        <SprintLeagueEditor spaceId={space_id} />
      </div>
    </SSRHydration>
  );
}
