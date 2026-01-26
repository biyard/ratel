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
import { ChevronDown } from 'lucide-react';
import { useContext, useEffect } from 'react';
import TeamCreationPopup from '../_popups/team-creation-popup';
import type { Team } from '@/features/teams/types/team';
import { NavLink } from 'react-router';
import { route } from '@/route';
import { useAuth } from '@/lib/contexts/auth-context';
import { TeamContext } from '@/lib/contexts/team-context';
import { useTranslation } from 'react-i18next';
import { useUserInfo } from '@/hooks/use-user-info';

export interface TeamSelectorProps {
  onSelect?: (index: number) => void;
  team?: Team;
}

export default function TeamSelector({ onSelect, team }: TeamSelectorProps) {
  const { t } = useTranslation('Home');
  const popup = usePopup();
  const { logout } = useAuth();
  const { teams, setSelectedTeam } = useContext(TeamContext);
  const userInfo = useUserInfo();
  const { isLoading } = userInfo;

  useEffect(() => {
    if (team) {
      const index = teams.findIndex((t) => t.pk === team.pk);
      if (index !== -1) {
        setSelectedTeam(index);
      }
    }
  }, [team, teams, setSelectedTeam]);

  if (isLoading || teams.length === 0) {
    return <div />;
  }

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <button
          className="w-full flex items-center justify-between px-2 py-2 focus:outline-none"
          data-pw="team-selector-trigger"
        >
          <span className="font-bold text-[18px] text-text-primary truncate">
            {team.nickname}
          </span>
          <ChevronDown size={16} className="text-text-primary" />
        </button>
      </DropdownMenuTrigger>

      <DropdownMenuContent className="w-[var(--radix-dropdown-menu-trigger-width)] bg-background">
        <DropdownMenuLabel className="text-text-primary">
          {t('teams')}
        </DropdownMenuLabel>

        <div className="max-h-[300px] overflow-y-auto pr-1 -mr-1">
          <DropdownMenuGroup>
            {teams.map((team, index) =>
              team.nickname !== '' ? (
                <DropdownMenuItem
                  key={`team-select-${index}-${team.username || team.pk}`}
                  className="focus:bg-accent focus:text-text-primary [&_svg:not([class*='size-'])]:size-4 w-full flex flex-row items-center gap-2 px-2 py-2 hover:bg-hover"
                  asChild
                >
                  <NavLink
                    to={
                      index === 0
                        ? route.home()
                        : route.teamByUsername(team.username)
                    }
                    className="flex items-center gap-2"
                    onClick={() => {
                      setSelectedTeam(index);
                      onSelect?.(index);
                    }}
                  >
                    {team.profile_url ? (
                      <img
                        src={team.profile_url}
                        alt={team.nickname}
                        width={24}
                        height={24}
                        className="w-6 h-6 rounded-full object-cover object-top"
                      />
                    ) : (
                      <div className="w-6 h-6 rounded-full border border-neutral-600 bg-neutral-600" />
                    )}
                    <span className="text-text-primary">{team.nickname}</span>
                  </NavLink>
                </DropdownMenuItem>
              ) : null,
            )}
          </DropdownMenuGroup>
        </div>

        <DropdownMenuSeparator />

        <DropdownMenuGroup>
          <DropdownMenuItem
            onClick={() =>
              popup.open(<TeamCreationPopup />).withTitle(t('create_new_team'))
            }
            data-pw="open-team-creation-popup"
          >
            <span className="text-text-primary">{t('create_team')}</span>
          </DropdownMenuItem>

          <DropdownMenuItem
            onClick={() => {
              logout();
              userInfo.refetch();
            }}
          >
            <span className="text-text-primary">{t('logout')}</span>
          </DropdownMenuItem>
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
