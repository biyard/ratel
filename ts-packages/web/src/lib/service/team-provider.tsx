'use client';

import { useState, useMemo, useEffect } from 'react';
import type { Team } from '@/features/teams/types/team';
import { useSuspenseUserInfo } from '@/hooks/use-user-info';
import { TeamContext } from '@/lib/contexts/team-context';
import { useUserInfo } from '@/hooks/use-user-info';
import { UserType } from '../api/ratel/users.v3';

export const TeamProvider = ({ children }: { children: React.ReactNode }) => {
  const { data } = useUserInfo();

  if (!data) {
    return <>{children}</>;
  }

  return <TeamAuthProvider>{children}</TeamAuthProvider>;
};

export const TeamAuthProvider = ({
  children,
}: {
  children: React.ReactNode;
}) => {
  const { data: user } = useSuspenseUserInfo();
  const [selectedIndex, setSelectedTeam] = useState(0);
  const [teams, setTeams] = useState<Team[]>([]);

  useEffect(() => {
    if (user) {
      const userAsTeam: Team = {
        pk: user.pk,
        nickname: user.nickname,
        username: user.username,
        profile_url: user.profile_url,
        created_at: 0,
        updated_at: 0,
        html_contents: user.description || '',
        user_type: user.user_type,
      };

      const userTeamsAsTeams: Team[] = (user.teams ?? []).map((team) => ({
        pk: team.team_pk,
        nickname: team.nickname,
        username: team.username,
        profile_url: team.profile_url,
        created_at: 0,
        updated_at: 0,
        html_contents: '',
        user_type: UserType.Team,
      }));

      setTeams([userAsTeam, ...userTeamsAsTeams]);
    }
  }, [user]);

  const selectedTeam = useMemo(() => {
    return teams[selectedIndex];
  }, [teams, selectedIndex]);

  const updateSelectedTeam = (updatedTeam: Team) => {
    const updatedTeams = teams.map((team) =>
      team.pk === updatedTeam.pk ? { ...team, ...updatedTeam } : team,
    );
    setTeams(updatedTeams);
    setSelectedTeam(0);
  };

  return (
    <TeamContext.Provider
      value={{
        teams,
        selectedTeam,
        selectedIndex,
        setSelectedTeam,
        updateSelectedTeam,
      }}
    >
      {children}
    </TeamContext.Provider>
  );
};
