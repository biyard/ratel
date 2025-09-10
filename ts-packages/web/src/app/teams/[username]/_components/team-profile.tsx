import React from 'react';
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
import FollowButton from './follow-button';
import UnFollowButton from './unfollow-button';

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
  const enableFollowbutton = team.id != userInfo.id;

  const handleFollow = async (userId: number) => {
    await post(ratelApi.networks.follow(userId), followRequest());
  };

  const handleUnFollow = async (userId: number) => {
    await post(ratelApi.networks.unfollow(userId), unfollowRequest());
  };

  return (
    <div className="flex flex-col gap-5 px-4 py-5 rounded-[10px] bg-card-bg border border-card-border">
      <TeamSelector team={team} />
      <div className="relative">
        {team.profile_url && team.profile_url !== '' ? (
          <Image
            src={team?.profile_url}
            alt={team?.nickname ?? 'team profile'}
            width={80}
            height={80}
            className="rounded-full border-2 object-cover object-top w-20 h-20"
          />
        ) : (
          <div className="w-20 h-20 rounded-full bg-profile-bg" />
        )}
      </div>

      <div className="font-medium text-text-primary">{team.nickname}</div>

      <div
        id="user-profile-description"
        className="text-xs text-desc-text"
        dangerouslySetInnerHTML={{ __html: team.html_contents }}
      />

      {enableFollowbutton ? (
        !isFollowing ? (
          <FollowButton
            onClick={async () => {
              try {
                await handleFollow(team.id);
                await data.refetch();
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
                await data.refetch();
                showSuccessToast('success to unfollow user');
              } catch (err) {
                showErrorToast('failed to unfollow user');
                logger.error('failed to unfollow user with error: ', err);
              }
            }}
          />
        )
      ) : null}
    </div>
  );
}
