import FeedCard from '@/components/feed-card';
import { Col } from '@/components/ui/col';

import PromotionCard from './_components/promotion-card';
import Suggestions from './_components/suggestions';
import { useHomeController } from './use-home-controller';
import {
  CreatePostButton,
  FeedEndMessage,
} from '@/features/drafts/components/list-drafts';
import Card from '@/components/card';
import { useNavigate } from 'react-router';
import { useCreatePostMutation } from '@/features/posts/hooks/use-create-post-mutation';
import { route } from '@/route';

export const SIZE = 10;

export default function HomePage() {
  const ctrl = useHomeController();
  const navigate = useNavigate();

  const createDraft = useCreatePostMutation().mutateAsync;

  if (ctrl.isLoading) {
    return (
      <div className="flex flex-row w-full h-fit justify-start items-center px-[16px] py-[20px] border border-gray-500 rounded-[8px] font-medium text-base text-gray-500">
        Loading...
      </div>
    );
  }

  let feedSection = (
    <Col className="flex-1 flex max-mobile:px-[10px]">
      <Col className="flex-1">
        {ctrl.posts.map((post) => (
          <FeedCard key={`feed-${post.pk}`} post={post} />
        ))}

        <div ref={ctrl.observerRef} />
        {!ctrl.hasNext && (
          <FeedEndMessage msg="You have reached the end of your feed." />
        )}
      </Col>
    </Col>
  );

  if (ctrl.posts.length === 0) {
    feedSection = (
      <div className="flex flex-row w-full h-fit justify-start items-center px-[16px] py-[20px] border border-gray-500 rounded-[8px] font-medium text-base text-gray-500">
        No posts available
      </div>
    );
  }

  return (
    <div className="flex-1 flex relative">
      {feedSection}

      <div
        className="flex-col w-70 pl-4 max-tablet:fixed bottom-4 max-tablet:right-4 max-tablet:z-50 max-tablet:pl-0"
        aria-label="Sidebar"
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
        <div className="max-tablet:hidden">
          {ctrl.topPromotion && (
            <Card variant="secondary">
              <PromotionCard promotion={ctrl.topPromotion} />
            </Card>
          )}

          {/* TODO: implement with v3
            <div className="mt-[10px]">
          <News />
        </div> */}
          <div className="mt-[10px]">
            <Suggestions />
          </div>
        </div>
      </div>
    </div>
  );
}
