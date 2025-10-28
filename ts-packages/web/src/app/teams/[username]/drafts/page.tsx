import { FeedStatus } from '@/features/posts/types/post';

import { useTeamDetailByUsername } from '@/features/teams/hooks/use-team';
import { useDeletePostMutation } from '@/features/posts/hooks/use-delete-post-mutation';
import useInfiniteTeamDrafts from '@/features/drafts/hooks/use-team-drafts';
import { useCreatePostMutation } from '@/features/posts/hooks/use-create-post-mutation';
import { useNavigate } from 'react-router';
import ListDrafts, {
  CreatePostButton,
} from '@/features/drafts/components/list-drafts';
import { route } from '@/route';

export default function TeamDraftPage({ username }: { username: string }) {
  const teamQuery = useTeamDetailByUsername(username);
  const navigate = useNavigate();

  const team = teamQuery.data;
  const teamPk = team?.id || '';

  const {
    data: drafts,
    fetchNextPage,
    hasNextPage,
    isFetchingNextPage,
  } = useInfiniteTeamDrafts(teamPk);

  const createDraft = useCreatePostMutation().mutateAsync;

  const handleRemoveDraft = useDeletePostMutation(
    username,
    FeedStatus.Draft,
  ).mutateAsync;

  if (teamQuery.isLoading) {
    return <div className="flex justify-center p-8">Loading drafts...</div>;
  }

  if (teamQuery.error) {
    return (
      <div className="flex justify-center p-8 text-red-500">
        Error loading team
      </div>
    );
  }

  if (!team) {
    return (
      <div className="flex justify-center p-8 text-red-500">Team not found</div>
    );
  }

  const flattedDrafts = drafts?.pages.flatMap((page) => page.items) ?? [];
  return (
    <div className="flex flex-1 relative">
      <div className="flex-1 flex max-mobile:px-[10px]">
        <ListDrafts
          drafts={flattedDrafts}
          fetchNextPage={fetchNextPage}
          hasNextPage={hasNextPage}
          isFetchingNextPage={isFetchingNextPage}
          onDelete={handleRemoveDraft}
        />
      </div>

      <div
        className={`h-fit max-tablet:fixed max-tablet:bottom-4 max-tablet:right-4 tablet:w-80 tablet:pl-4 tablet:static`}
      ></div>

      <CreatePostButton
        onClick={async () => {
          try {
            // pass an empty variables object as the mutation requires at least one argument
            const draft = await createDraft({ teamPk: team.id });
            navigate(route.draftEdit(draft.post_pk));
          } catch (error) {
            console.error('Error creating draft:', error);
          }
        }}
      />
    </div>
  );
}
