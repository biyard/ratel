import FeedCard from '@/components/feed-card';
import { Col } from '@/components/ui/col';

import { UserType } from '@/lib/api/models/user';

import { type Space } from '@/lib/api/models/spaces';
import FeedEndMessage from './_components/feed-end-message';
import CreatePostButton from './_components/create-post-button';
import PromotionCard from './_components/promotion-card';
import Suggestions from './_components/suggestions';
import DisableBorderCard from './_components/disable-border-card';
import { useHomeController } from './use-home-controller';

export const SIZE = 10;

export interface Post {
  id: number;
  industry: string;
  title: string;
  contents: string;
  url?: string;
  author_id: number;
  author_profile_url: string;
  author_name: string;
  author_type: UserType;
  space_id?: number;
  space_type?: number;
  likes: number;
  is_liked: boolean;
  comments: number;
  rewards: number;
  shares: number;
  created_at: number;
  onboard: boolean;

  spaces: Space[];
}

export default function HomePage() {
  const ctrl = useHomeController();

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
        {!ctrl.hasNext && <FeedEndMessage />}
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

      {ctrl.close && (
        <div className="hidden max-tablet:!block fixed bottom-4 right-4 z-50">
          <CreatePostButton />
        </div>
      )}

      <aside className="w-70 pl-4 max-tablet:!hidden" aria-label="Sidebar">
        <CreatePostButton />

        {ctrl.topPromotion && (
          <DisableBorderCard>
            <PromotionCard promotion={ctrl.topPromotion} />
          </DisableBorderCard>
        )}

        {/* TODO: implement with v3
            <div className="mt-[10px]">
          <News />
        </div> */}
        <div className="mt-[10px]">
          <Suggestions />
        </div>
      </aside>
    </div>
  );
}
