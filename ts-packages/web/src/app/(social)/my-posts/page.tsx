import { useCallback } from 'react';
import { Col } from '@/components/ui/col';
import FeedCard from '@/components/feed-card';
import { useObserver } from '@/hooks/use-observer';
import useInfiniteMyPosts from './_hooks/use-my-posts';
import {
  CreatePostButton,
  FeedEndMessage,
} from '@/features/drafts/components/list-drafts';
import { useCreatePostMutation } from '@/features/posts/hooks/use-create-post-mutation';
import { useNavigate } from 'react-router';
import { route } from '@/route';

export default function MyPostsPage() {
  const { data, fetchNextPage, hasNextPage, isFetchingNextPage } =
    useInfiniteMyPosts();

  const navigate = useNavigate();

  const createDraft = useCreatePostMutation().mutateAsync;

  const handleIntersect = useCallback(() => {
    if (hasNextPage && !isFetchingNextPage) {
      fetchNextPage();
    }
  }, [fetchNextPage, hasNextPage, isFetchingNextPage]);
  const observerRef = useObserver<HTMLDivElement>(handleIntersect, {
    threshold: 1,
  });

  if (data.pages.length === 0) {
    return (
      <div className="flex flex-row w-full h-fit justify-start items-center px-[16px] py-[20px] border border-gray-500 rounded-[8px] font-medium text-base text-gray-500">
        No drafts available
      </div>
    );
  }
  const flattedPosts = data?.pages.flatMap((page) => page.items) ?? [];

  return (
    <div className="flex-1 flex relative">
      <Col className="flex-1 flex max-mobile:px-[10px]">
        <Col className="flex-1">
          {flattedPosts.map((post) => (
            <FeedCard key={`feed-${post.pk}`} post={post} />
          ))}

          <div ref={observerRef} />
          {!hasNextPage && (
            <FeedEndMessage msg="You have reached the end of your feed." />
          )}
        </Col>
      </Col>

      <div
        className={`h-fit max-tablet:fixed max-tablet:bottom-4 max-tablet:right-4 tablet:w-80 tablet:pl-4 tablet:static`}
      >
        <CreatePostButton
          onClick={async () => {
            try {
              const draft = await createDraft({});
              navigate(route.draftEdit(draft.post_pk));
            } catch (error) {
              console.error('Error creating draft:', error);
            }
          }}
        />
      </div>
    </div>
  );
}
