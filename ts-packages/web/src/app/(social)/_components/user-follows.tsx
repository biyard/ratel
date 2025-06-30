import React from 'react';
import Link from 'next/link';
import { route } from '@/route';
import { RelationType } from '@/types/relation-type';

export interface UserFollowsProps {
  followers_count: number;
  followings_count: number;
}

export default function UserFollows({
  followers_count,
  followings_count,
}: UserFollowsProps) {
  return (
    <div className="flex flex-row w-full justify-around items-center gap-[20px] max-tablet:gap-[10px]">
      <Link
        className="flex flex-col w-fit justify-start items-center gap-[2px] text-c-wg-50 hover:text-white"
        href={route.myFollower(RelationType.FOLLOWER)}
      >
        <div className="font-bold text-sm">
          {followers_count.toLocaleString()}
        </div>
        <div className="font-medium text-xs">Followers</div>
      </Link>

      <Link
        className="flex flex-col w-fit justify-start items-center gap-[2px] text-c-wg-50 hover:text-white"
        href={route.myFollower(RelationType.FOLLOWING)}
      >
        <div className="font-bold text-sm">
          {followings_count.toLocaleString()}
        </div>
        <div className="font-medium text-xs">Followings</div>
      </Link>
    </div>
  );
}
