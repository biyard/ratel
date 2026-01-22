'use client';
import { useContext, useMemo } from 'react';
import { TeamContext } from '@/lib/contexts/team-context';
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
import { NavLink } from 'react-router';
import { route } from '@/route';
import TeamCreationPopup from '@/app/(social)/_popups/team-creation-popup';
import { useTranslation } from 'react-i18next';
import { useUserInfo } from '@/hooks/use-user-info';

interface ProfileProps {
  profileUrl?: string;
  name?: string;
}

export default function Profile({ profileUrl, name }: ProfileProps) {
  const { t } = useTranslation('Home');
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
        <button className="w-fit flex items-center justify-between">
          <div className="flex flex-col items-center justify-center p-2.5 group">
            {profileUrl && profileUrl !== '' ? (
              <img
                src={profileUrl}
                alt="User Profile"
                className="w-6 h-6 rounded-full object-cover w-6 h-6"
              />
            ) : (
              <div className="w-6 h-6 bg-neutral-500 rounded-full" />
            )}
            <span className="text-menu-text group-hover:text-menu-text/80 text-[15px] font-medium transition-colors max-w-20 truncate">
              {name || 'Unknown User'}
            </span>
          </div>
        </button>
      </DropdownMenuTrigger>

      <DropdownMenuContent
        align="end"
        className="w-[250px] h-fit rounded-lg border border-primary bg-background p-[10px] bg-bg z-999"
      >
        <DropdownMenuLabel className="text-xs text-text-primary-muted px-2 py-1">
          {t('teams')}
        </DropdownMenuLabel>

        <div className="max-h-[300px] overflow-y-auto pr-2 -mr-2">
          <DropdownMenuGroup>
            {teams.map((team, index) => (
              <DropdownMenuItem
                key={`team-select-menu-${team.pk}`}
                asChild
                className="w-full px-2 py-1.5 hover:bg-hover rounded-md cursor-pointer focus-visible:outline-none"
              >
                <NavLink
                  to={
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
                  {team.profile_url ? (
                    <img
                      src={team.profile_url}
                      alt={team.nickname}
                      className="w-6 h-6 rounded-full object-cover object-top"
                    />
                  ) : (
                    <div className="w-6 h-6 bg-neutral-600 rounded-full" />
                  )}
                  <span className="text-sm text-text-primary-muted truncate">
                    {team.nickname}
                  </span>
                </NavLink>
              </DropdownMenuItem>
            ))}
          </DropdownMenuGroup>
        </div>

        <DropdownMenuSeparator className="my-2 bg-neutral-700 light:bg-[#e5e5e5] h-px" />

        <DropdownMenuGroup>
          <DropdownMenuItem
            onClick={() =>
              popup.open(<TeamCreationPopup />).withTitle('Create a new team')
            }
            className="w-full px-2 py-1.5 hover:bg-hover rounded-md text-sm text-text-primary-muted cursor-pointer focus-visible:outline-none"
          >
            <span id="create_team">{t('create_team')}</span>
          </DropdownMenuItem>

          <DropdownMenuItem
            onClick={() => {
              logout();
              userInfo.refetch();
            }}
            className="w-full px-2 py-1.5 hover:bg-hover rounded-md text-sm text-text-primary-muted cursor-pointer focus-visible:outline-none"
          >
            <span>{t('logout')}</span>
          </DropdownMenuItem>
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
