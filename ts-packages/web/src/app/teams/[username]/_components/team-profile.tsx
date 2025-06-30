import React, { useState } from 'react';
import Image from 'next/image';
import { Team } from '@/lib/api/models/team';
import TeamSelector from '@/app/(social)/_components/team-selector';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import {
  followRequest,
  unfollowRequest,
} from '@/lib/api/models/networks/follow';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { logger } from '@/lib/logger';

export interface TeamProfileProps {
  team?: Team;
}

export default function TeamProfile({ team }: TeamProfileProps) {
  const { post } = useApiCall();
  const data = useSuspenseUserInfo();

  if (!team) {
    return <div></div>;
  }

  const userInfo = data.data;
  const followings = userInfo.followings;

  const isFollowing = followings.some((f: { id: number }) => f.id === team.id);

  const handleFollow = async (userId: number) => {
    await post(ratelApi.networks.follow(userId), followRequest());
  };

  const handleUnFollow = async (userId: number) => {
    await post(ratelApi.networks.unfollow(userId), unfollowRequest());
  };

  return (
    <div className="flex flex-col gap-5 px-4 py-5 rounded-[10px] bg-component-bg">
      <TeamSelector team={team} />
      <div className="relative">
        <Image
          src={team?.profile_url || '/default-profile.png'}
          alt={team?.nickname ?? 'team profile'}
          width={80}
          height={80}
          className="rounded-full border-2 object-cover object-top w-[80px] h-[80px]"
        />
      </div>

      <div className="font-medium">{team.nickname}</div>

      <div
        id="user-profile-description"
        className="text-xs text-gray-400"
        dangerouslySetInnerHTML={{ __html: team.html_contents }}
      />

      {!isFollowing ? (
        <FollowButton
          onClick={async () => {
            try {
              await handleFollow(team.id);
              data.refetch();

              showSuccessToast('success to follow user');
            } catch (err) {
              showErrorToast('failed to follow user');
              logger.error('failed to follow user with error: ', err);
            }
          }}
        />
      ) : (
        <UnFollowButton
          onClick={async () => {
            try {
              await handleUnFollow(team.id);
              data.refetch();

              showSuccessToast('success to unfollow user');
            } catch (err) {
              showErrorToast('failed to unfollow user');
              logger.error('failed to unfollow user with error: ', err);
            }
          }}
        />
      )}
    </div>
  );
}

function UnFollowButton({ onClick }: { onClick: () => void }) {
  const [isHovered, setIsHovered] = useState(false);

  return (
    <div
      className="cursor-pointer flex flex-row w-fit h-fit px-[10px] py-[5px] bg-transparent border border-neutral-700 hover:border-[#ff4d4f] hover:bg-[#ffe3e3] rounded-[50px]"
      onClick={onClick}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
    >
      <div
        className={`font-bold  ${isHovered ? 'text-[#ff4d4f]' : 'text-neutral-700'} text-xs`}
      >
        {isHovered ? 'Unfollow' : 'Following'}
      </div>
    </div>
  );
}

function FollowButton({ onClick }: { onClick: () => void }) {
  return (
    <div
      className="cursor-pointer flex flex-row w-fit h-fit px-[10px] py-[5px] bg-white hover:bg-gray-300 rounded-[50px]"
      onClick={() => {
        onClick();
      }}
    >
      <div className="font-bold text-[#000203] text-xs">Follow</div>
    </div>
  );
}
