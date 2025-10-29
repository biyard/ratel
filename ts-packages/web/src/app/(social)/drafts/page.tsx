import { useSuspenseUserInfo } from '@/hooks/use-user-info';

import { FeedStatus } from '@/features/posts/types/post';
import { useDeletePostMutation } from '@/features/posts/hooks/use-delete-post-mutation';
import useInfiniteMyDrafts from '../../../features/drafts/hooks/use-my-drafts';
import ListDrafts, {
  CreatePostButton,
} from '@/features/drafts/components/list-drafts';
import { Row } from '@/components/ui/row';
import { Col } from '@/components/ui/col';

export default function MyDraftPage() {
  const { data: user } = useSuspenseUserInfo();
  const {
    data: drafts,
    fetchNextPage,
    hasNextPage,
    isFetchingNextPage,
  } = useInfiniteMyDrafts();

  const username = user?.username || '';

  const handleRemoveDraft = useDeletePostMutation(
    username,
    FeedStatus.Draft,
  ).mutateAsync;

  if (!user) return null;

  const flattedDrafts = drafts?.pages.flatMap((page) => page.items) ?? [];
  return (
    <Row>
      <Col>
        {/* <div className="flex flex-1 max-mobile:px-[10px]"> */}
        <ListDrafts
          drafts={flattedDrafts}
          fetchNextPage={fetchNextPage}
          hasNextPage={hasNextPage}
          isFetchingNextPage={isFetchingNextPage}
          onDelete={handleRemoveDraft}
        />
      </Col>
      <div
        className={`h-fit max-tablet:fixed max-tablet:bottom-4 max-tablet:right-4 tablet:w-80 tablet:pl-4 tablet:static`}
      >
        <CreatePostButton />
      </div>
    </Row>
  );
}
