'use client';

import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { usePopup } from '@/lib/contexts/popup-service';
import { logger } from '@/lib/logger';
import { ChevronDown } from 'lucide-react';
import React, { useContext, useEffect } from 'react';
import TeamCreationPopup from '../_popups/team-creation-popup';
import { useUserInfo } from '@/lib/api/hooks/users';
import { Team } from '@/lib/api/models/team';
import Link from 'next/link';
import { route } from '@/route';
import { useAuth } from '@/lib/contexts/auth-context';
import { TeamContext } from '@/lib/contexts/team-context';
import Image from 'next/image';
import { useTranslations } from 'next-intl';

export interface TeamSelectorProps {
  onSelect?: (index: number) => void;
  team?: Team;
}

export default function TeamSelector({ onSelect, team }: TeamSelectorProps) {
  const t = useTranslations('Home');
  const popup = usePopup();
  const { logout } = useAuth();
  const { teams, selectedIndex, setSelectedTeam } = useContext(TeamContext);
  const userInfo = useUserInfo();
  const { isLoading } = userInfo;

  useEffect(() => {
    if (team) {
      const index = teams.findIndex((t) => t.id === team.id);
      if (index !== -1) {
        setSelectedTeam(index);
      }
    }
  }, [team, teams, setSelectedTeam]);

  if (isLoading || teams.length === 0) {
    return <div />;
  }

  logger.debug('TeamSelector groups:', teams);

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <button className="w-full flex items-center justify-between px-2 py-2 focus:outline-none">
          <span className="font-bold text-[18px] text-foreground">
            {teams[selectedIndex].nickname}
          </span>
          <ChevronDown size={16} className='text-foreground' />
        </button>
      </DropdownMenuTrigger>

      <DropdownMenuContent className="w-[var(--radix-dropdown-menu-trigger-width)] bg-background">
        <DropdownMenuLabel className="text-foreground-muted">{t('teams')}</DropdownMenuLabel>
        <DropdownMenuGroup>
          {teams.map((team, index) => {
            return team.nickname != '' ? (
              <DropdownMenuItem
                className="focus:bg-accent focus:text-accent-foreground data-[variant=destructive]:text-destructive data-[variant=destructive]:focus:bg-destructive/10 dark:data-[variant=destructive]:focus:bg-destructive/20 data-[variant=destructive]:focus:text-destructive data-[variant=destructive]:*:[svg]:!text-destructive [&_svg:not([class*='text-'])]:text-muted-foreground relative flex cursor-default items-center gap-2 rounded-sm px-2 py-1.5 text-sm outline-hidden select-none data-[disabled]:pointer-events-none data-[disabled]:opacity-50 data-[inset]:pl-8 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 w-full flex flex-row items-center gap-2 px-2 py-2 hover:bg-neutral-800 cursor-pointer"
                key={`team-select-menu-${team.id}`}
                asChild
              >
                <Link
                  href={
                    index === 0
                      ? route.home()
                      : route.teamByUsername(team.username)
                  }
                  className="flex items-center gap-2"
                  onClick={() => {
                    setSelectedTeam(index);
                    onSelect!(index);
                  }}
                >
                  {team.profile_url && team.profile_url !== '' ? (
                    <Image
                      src={team.profile_url}
                      alt={team.nickname}
                      width={24}
                      height={24}
                      className="w-6 h-6 rounded-full object-cover object-top"
                    />
                  ) : (
                    <div className="w-6 h-6 rounded-full border border-neutral-500 bg-neutral-600" />
                  )}
                  <span className="text-foreground-muted">{team.nickname}</span>
                </Link>
              </DropdownMenuItem>
            ) : (
              <></>
            );
          })}
        </DropdownMenuGroup>
        <DropdownMenuSeparator />
        <DropdownMenuGroup>
          <DropdownMenuItem
            onClick={() => {
              logger.debug('Create team clicked');
              popup.open(<TeamCreationPopup />).withTitle(t('create_new_team'));
            }}
          >
            <span className="text-foreground-muted">{t('create_team')}</span>
          </DropdownMenuItem>

          <DropdownMenuItem
            onClick={() => {
              logger.debug('Create team clicked');
              logout();
              userInfo.refetch();
            }}
          >
            <span className="text-foreground-muted">{t('logout')}</span>
          </DropdownMenuItem>
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
