import FeedCard from '@/components/feed-card';
import { Col } from '@/components/ui/col';

import PromotionCard from './_components/promotion-card';
import { useHomeController } from './use-home-controller';
import {
  CreatePostButton,
  FeedEndMessage,
} from '@/features/drafts/components/list-drafts';
import Card from '@/components/card';

export const SIZE = 10;

export default function HomePage() {
  const ctrl = useHomeController();

  if (ctrl.isLoading) {
    return (
      <div className="flex flex-row justify-start items-center w-full text-base font-medium text-gray-500 border border-gray-500 h-fit px-[16px] py-[20px] rounded-[8px]">
        Loading...
      </div>
    );
  }

  let feedSection = (
    <Col className="flex flex-1 max-mobile:px-[10px]">
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
      <div className="flex flex-row justify-start items-center w-full text-base font-medium text-gray-500 border border-gray-500 h-fit px-[16px] py-[20px] rounded-[8px]">
        No posts available
      </div>
    );
  }

  return (
    <div className="flex relative flex-1 gap-4">
      {feedSection}

      <div
        className="flex flex-col gap-2.5 w-70 max-tablet:fixed max-tablet:bottom-4 max-tablet:right-4 max-tablet:z-50 max-tablet:pl-0"
        aria-label="Sidebar"
      >
        <CreatePostButton />

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
          {/* <div className="mt-[10px]">
            <Suggestions />
          </div> */}
        </div>
      </div>
    </div>
  );
}
