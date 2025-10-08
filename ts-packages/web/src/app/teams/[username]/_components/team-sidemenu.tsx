'use client';

import { useContext, useMemo } from 'react';
// import { Users, MessageSquare } from 'lucide-react';
// import type { Team } from '@/lib/api/models/team';
import TeamProfile from './team-profile';
import { Link } from 'react-router';
import { route } from '@/route';
import {
  Home,
  UserGroup,
  Settings,
  EditContent,
  Folder,
} from '@/components/icons';
import { TeamContext } from '@/lib/contexts/team-context';
import { useTranslation } from 'react-i18next';
import { useUserByUsername } from '@/app/(social)/_hooks/use-user';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import FollowButton from './follow-button';
import UnFollowButton from './unfollow-button';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { logger } from '@/lib/logger';
import {
  followRequest,
  unfollowRequest,
} from '@/lib/api/models/networks/follow';
import { GroupPermission } from '@/lib/api/models/group';
import { usePermission } from '@/app/(social)/_hooks/use-permission';

export interface TeamSidemenuProps {
  username: string;
}

export default function TeamSidemenu({ username }: TeamSidemenuProps) {
  const { post } = useApiCall();
  const { t } = useTranslation('Team');
  const { teams } = useContext(TeamContext);
  const team = useMemo(() => {
    return teams.find((t) => t.username === username);
  }, [teams, username]);

  const { data: user } = useUserByUsername(username);
  // TODO: Implement followings in v3 API - using placeholder for now
  const followings: never[] = [];
  const isFollowing = followings.some((f: { id: number }) => f.id === user.id);

  const handleFollow = async (userId: number) => {
    await post(ratelApi.networks.follow(userId), followRequest());
  };

  const handleUnFollow = async (userId: number) => {
    await post(ratelApi.networks.unfollow(userId), unfollowRequest());
  };

  // TODO: Update to use v3 permissions with username instead of id
  const writePostPermission =
    usePermission(team?.username ?? '', GroupPermission.WritePosts).data
      .has_permission ?? false;

  const updateGroupPermission =
    usePermission(team?.username ?? '', GroupPermission.UpdateGroup).data
      .has_permission ?? false;

  if (!team && !user) {
    return <></>;
  }

  if (!team) {
    return (
      <div className="flex flex-col gap-5 px-4 py-5 rounded-[10px] bg-card-bg border border-card-border min-w-[250px] h-fit">
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
            <div className="w-20 h-20 rounded-full bg-profile-bg" />
          )}
        </div>

        <div className="font-medium text-text-primary">{user.nickname}</div>

        <div
          id="user-profile-description"
          className="text-xs text-desc-text"
          dangerouslySetInnerHTML={{ __html: user.html_contents }}
        />

        {!isFollowing ? (
          <FollowButton
            onClick={async () => {
              try {
                await handleFollow(user.id);
                // TODO: Implement refetch after follow/unfollow in v3

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
                await handleUnFollow(user.id);
                // TODO: Implement refetch after follow/unfollow in v3

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

  return (
    <div className="w-64 flex flex-col max-mobile:!hidden gap-2.5">
      <TeamProfile team={team} />

      <nav className="py-5 px-3 w-full rounded-[10px] bg-card-bg border border-card-border">
        <Link
          to={route.teamByUsername(team.username)}
          className="sidemenu-link text-text-primary [&>path]:stroke-[#737373]"
        >
          <Home className="w-6 h-6" />
          <span>{t('home')}</span>
        </Link>
        {writePostPermission ? (
          <Link
            to={route.teamDrafts(team.username)}
            className="sidemenu-link text-text-primary"
          >
            <EditContent className="w-6 h-6 [&>path]:stroke-[#737373]" />
            <span>{t('drafts')}</span>
          </Link>
        ) : (
          <></>
        )}
        <Link
          to={route.teamGroups(team.username)}
          className="sidemenu-link text-text-primary "
        >
          <Folder className="w-6 h-6 [&>path]:stroke-[#737373]" />
          <span>{t('manage_group')}</span>
        </Link>
        <Link
          to={route.teamMembers(team.username)}
          className="sidemenu-link text-text-primary"
        >
          <UserGroup className="w-6 h-6 [&>path]:stroke-[#737373]" />
          <span>{t('members')}</span>
        </Link>
        {updateGroupPermission ? (
          <Link
            to={route.teamSettings(team.username)}
            className="sidemenu-link text-text-primary"
          >
            <Settings className="w-6 h-6" />
            <span>{t('settings')}</span>
          </Link>
        ) : (
          <></>
        )}
      </nav>

      {/* <nav className="mt-4 px-2">
        <div className="flex items-center gap-3 px-2 py-2 rounded-md hover:bg-gray-800">
          <div className="w-5 h-5 rounded-full border border-gray-500 flex items-center justify-center">
            <Users size={12} />
          </div>
          <span className="text-sm">Profile</span>
        </div>
        <div className="flex items-center gap-3 px-2 py-2 rounded-md hover:bg-gray-800">
          <div className="w-5 h-5 rounded-full border border-gray-500 flex items-center justify-center">
            <MessageSquare size={12} />
          </div>
          <span className="text-sm">Threads</span>
        </div>
        <div className="flex items-center gap-3 px-2 py-2 rounded-md hover:bg-gray-800">
          <div className="w-5 h-5 rounded-full border border-gray-500 flex items-center justify-center">
            <Users size={12} />
          </div>
          <span className="text-sm">Manage Group</span>
        </div>
        <div className="flex items-center gap-3 px-2 py-2 rounded-md hover:bg-gray-800">
          <div className="w-5 h-5 rounded-full border border-gray-500 flex items-center justify-center">
            <Users size={12} />
          </div>
          <span className="text-sm">Settings</span>
        </div>
      </nav> */}
    </div>
  );
}
