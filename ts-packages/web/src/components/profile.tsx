'use client';
import React, { useContext, useMemo } from 'react';
import Image from 'next/image';
import { TeamContext } from '@/lib/contexts/team-context';
import { useUserInfo } from '@/lib/api/hooks/users';
import { useAuth } from '@/lib/contexts/auth-context';
import { usePopup } from '@/lib/contexts/popup-service';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@radix-ui/react-dropdown-menu';
import Link from 'next/link';
import { route } from '@/route';
import { logger } from '@/lib/logger';
import TeamCreationPopup from '@/app/(social)/_popups/team-creation-popup';

interface ProfileProps {
  profileUrl?: string;
  name?: string;
}

export default function Profile({ profileUrl, name }: ProfileProps) {
  const { teams, selectedIndex, setSelectedTeam } = useContext(TeamContext);
  const team = useMemo(() => teams[selectedIndex], [teams, selectedIndex]);
  const userInfo = useUserInfo();
  const { logout } = useAuth();
  const popup = usePopup();

  if (!team) {
    return <div />;
  }

  const handleTeamSelect = (i: number) => {
    setSelectedTeam(i);
  };

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild className="focus-visible:outline-none">
        <button className="w-full flex items-center justify-between">
          <div className="flex flex-col items-center justify-center p-2.5 group">
            {profileUrl ? (
              <Image
                src={profileUrl || '/default-profile.png'}
                alt="User Profile"
                width={24}
                height={24}
                className="rounded-full object-cover"
              />
            ) : (
              <div className="w-[24px] h-[24px] bg-neutral-500 rounded-full" />
            )}

            <span className="text-neutral-500 group-hover:text-white text-[15px] font-medium transition-colors">
              {name || 'Unknown User'}
            </span>
          </div>
        </button>
      </DropdownMenuTrigger>

      <DropdownMenuContent
        align="end"
        className="w-[250px] h-fit rounded-lg border border-primary p-[10px] bg-bg z-20"
      >
        <DropdownMenuLabel className="text-xs text-neutral-400 px-2 py-1">
          Teams
        </DropdownMenuLabel>

        <DropdownMenuGroup>
          {teams.map((team, index) => (
            <DropdownMenuItem
              key={`team-select-menu-${team.id}`}
              asChild
              className="w-full px-2 py-1.5 hover:bg-transparent rounded-md cursor-pointer focus-visible:outline-none"
            >
              <Link
                href={
                  index === 0
                    ? route.home()
                    : route.teamByUsername(team.username)
                }
                className="flex items-center gap-2 w-full"
                onClick={() => {
                  setSelectedTeam(index);
                  handleTeamSelect(index);
                }}
              >
                <Image
                  src={team.profile_url || '/default-profile.png'}
                  alt={team.nickname}
                  width={24}
                  height={24}
                  className="rounded-full object-cover object-top"
                />
                <span className="text-sm text-white truncate">
                  {team.nickname}
                </span>
              </Link>
            </DropdownMenuItem>
          ))}
        </DropdownMenuGroup>

        <DropdownMenuSeparator className="my-2 bg-neutral-700 h-px" />

        <DropdownMenuGroup>
          <DropdownMenuItem
            onClick={() => {
              logger.debug('Create team clicked');
              popup.open(<TeamCreationPopup />).withTitle('Create a new team');
            }}
            className="w-full px-2 py-1.5 hover:bg-transparent rounded-md text-sm text-white cursor-pointer focus-visible:outline-none"
          >
            <span>Create a team</span>
          </DropdownMenuItem>

          <DropdownMenuItem
            onClick={() => {
              logout();
              userInfo.refetch();
            }}
            className="w-full px-2 py-1.5 hover:bg-transparent rounded-md text-sm text-white cursor-pointer focus-visible:outline-none"
          >
            <span>Log out</span>
          </DropdownMenuItem>
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
