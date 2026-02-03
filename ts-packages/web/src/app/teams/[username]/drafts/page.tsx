// TeamDraftPage.tsx
import { FeedStatus } from '@/features/posts/types/post';
import { useDeletePostMutation } from '@/features/posts/hooks/use-delete-post-mutation';
import useInfiniteTeamDrafts from '@/features/drafts/hooks/use-team-drafts';
import ListDrafts, {
  CreatePostButton,
} from '@/features/drafts/components/list-drafts';
import { useParams } from 'react-router';
import { useSuspenseFindTeam } from '@/features/teams/hooks/use-find-team';

export default function TeamDraftPage() {
  const { username } = useParams<{ username: string }>();
  const { data: team } = useSuspenseFindTeam(username);
  const teamPk = team?.pk ?? '';

  const {
    data: drafts,
    fetchNextPage,
    hasNextPage,
    isFetchingNextPage,
  } = useInfiniteTeamDrafts(teamPk, FeedStatus.Draft);

  const deleteMutation = useDeletePostMutation(username, FeedStatus.Draft);
  const handleRemoveDraft = deleteMutation.mutateAsync;

  if (!team) {
    return (
      <div className="flex justify-center p-8 text-red-500">Team not found</div>
    );
  }

  const flattedDrafts = drafts?.pages.flatMap((p) => p.items) ?? [];

  return (
    <div className="flex relative flex-1 flex-row w-full">
      <div className="flex flex-1 flex-row max-mobile:px-2.5 w-full">
        <ListDrafts
          drafts={flattedDrafts}
          fetchNextPage={fetchNextPage}
          hasNextPage={hasNextPage}
          isFetchingNextPage={isFetchingNextPage}
          onDelete={handleRemoveDraft}
        />
      </div>

      <div
        className={`h-fit w-fit max-tablet:fixed max-tablet:bottom-4 max-tablet:right-4 tablet:w-80 tablet:pl-4 tablet:static`}
      >
        <CreatePostButton teamPk={teamPk} />
      </div>
    </div>
  );
}
