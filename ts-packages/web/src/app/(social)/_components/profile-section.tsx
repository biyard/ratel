'use client';
import React, { useContext, useMemo } from 'react';
import TeamSelector from './team-selector';
import UserTier from './UserTier';
import UserBadges from './user-badges';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { TeamContext } from '@/lib/contexts/team-context';
import UserFollows from './user-follows';

export default function ProfileSection() {
  const { data } = useSuspenseUserInfo();
  const user = data!;

  const { teams, selectedIndex, setSelectedTeam } = useContext(TeamContext);
  const team = useMemo(() => teams[selectedIndex], [teams, selectedIndex]);

  if (!team) {
    return <div />;
  }

  const handleTeamSelect = (i: number) => {
    setSelectedTeam(i);
  };

  return (
    <div className="flex flex-col gap-5 px-4 py-5 rounded-[10px] bg-component-bg">
      <TeamSelector onSelect={handleTeamSelect} team={team} />

      <div className="relative">
        {user?.profile_url && user?.profile_url !== '' ? (
          <img
            src={user?.profile_url}
            alt={user?.nickname ?? 'team profile'}
            width={80}
            height={80}
            className="w-20 h-20 rounded-full border-2 object-cover object-top"
          />
        ) : (
          <div className="w-20 h-20 rounded-full border border-neutral-500 bg-neutral-600" />
        )}
      </div>

      <div className="font-medium">{user.nickname}</div>

      <div
        id="user-profile-description"
        className="text-xs text-gray-400"
        dangerouslySetInnerHTML={{ __html: user.html_contents }}
      />

      <UserTier />
      <UserBadges badges={user.badges ? user.badges : []} />
      <UserFollows
        followers_count={user.followers_count}
        followings_count={user.followings_count}
      />
    </div>
  );
}
