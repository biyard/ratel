import { useSpaceBySpaceId } from '@/app/(social)/_hooks/use-spaces';
import { getTimeWithFormat } from '@/lib/time-utils';
import Clock from '@/assets/icons/clock.svg?react';

import useFeedById from '@/hooks/feeds/use-feed-by-id';
import BorderSpaceCard from '@/app/(social)/_components/border-space-card';

export default function SpaceSideMenu({ spaceId }: { spaceId: number }) {
  const { data: space } = useSpaceBySpaceId(spaceId);
  // TODO: Update space API to use string feed_id in v3
  const { data: feed } = useFeedById(space?.feed_id.toString());

  return (
    <div className="flex flex-col max-w-[250px] max-tablet:!hidden w-full gap-2.5">
      {feed.post.urls &&
      feed.post.urls.length > 0 &&
      feed.post.urls[0] !== '' ? (
        <img
          src={feed.post.urls[0]}
          alt={feed.post.title ?? ''}
          className="rounded-[10px] w-full h-[127px] object-cover object-top"
        />
      ) : (
        <div className="w-6 h-6 rounded-[10px] border border-neutral-500 bg-neutral-600" />
      )}

      <BorderSpaceCard>
        <div className="flex flex-col gap-5">
          <div className="flex flex-row gap-1 items-center">
            <Clock width={20} height={20} />
            <div className="font-bold text-neutral-500 text-sm/[14px]">
              Proposed
            </div>
          </div>

          <div className="font-medium text-white text-[15px]/[12px]">
            {getTimeWithFormat(space.created_at)}
          </div>
        </div>
      </BorderSpaceCard>
    </div>
  );
}
