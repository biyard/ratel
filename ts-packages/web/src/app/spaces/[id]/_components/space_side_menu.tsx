'use client';

import BlackBox from '@/app/(social)/_components/black-box';
import { useSpaceBySpaceId } from '@/app/(social)/_hooks/use-spaces';
import { getTimeWithFormat } from '@/lib/time-utils';
import React from 'react';
import Clock from '@/assets/icons/clock.svg';
import Image from 'next/image';
import useFeedById from '@/hooks/feeds/use-feed-by-id';

export default function SpaceSideMenu({ spaceId }: { spaceId: number }) {
  const { data: space } = useSpaceBySpaceId(spaceId);

  const { data: feed } = useFeedById(space?.feed_id);

  return (
    <div className="flex flex-col max-w-[250px] max-tablet:!hidden w-full gap-2.5">
      {feed.url && feed.url !== '' ? (
        <Image
          src={feed.url}
          alt={feed.title ?? ''}
          width={250}
          height={127}
          className="rounded-[10px] object-cover object-top"
        />
      ) : (
        <div className="w-6 h-6 rounded-[10px] border border-neutral-500 bg-neutral-600" />
      )}

      <BlackBox>
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
      </BlackBox>
    </div>
  );
}
