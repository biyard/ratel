'use client';

import React from 'react';
import SpaceHeader from '../_components/space_header';
import SpaceCouponProgress from '../_components/coupon-progress';
import SpaceContents from '../_components/space_contents';
import SpaceFiles from '../_components/space_files';
import { useRouter } from 'next/navigation';
import { useRedeemCode, useSpaceById } from '@/lib/api/ratel_api';
import { useCommitteeSpaceByIdContext } from './providers.client';

export default function CommitteeSpacePage() {
  const { spaceId } = useCommitteeSpaceByIdContext();
  const { data: space } = useSpaceById(spaceId);
  const redeem = useRedeemCode(spaceId);
  const router = useRouter();

  return (
    <div className="flex flex-col w-full justify-start items-start mb-4">
      <SpaceHeader
        title={space?.title ?? ''}
        status={space.status}
        userType={space?.author[0].user_type ?? 0}
        proposerImage={space?.author[0].profile_url ?? ''}
        proposerName={space?.author[0].nickname ?? ''}
        createdAt={space?.created_at}
        onback={() => {
          router.back();
        }}
      />
      <div className="flex flex-col w-full mt-7.5 gap-2.5">
        <SpaceCouponProgress progress={redeem.data?.used?.length || 0} />
        <SpaceContents htmlContents={space?.html_contents}></SpaceContents>
        <SpaceFiles files={space?.files} badges={space?.badges} />
      </div>
    </div>
  );
}
