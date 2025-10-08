'use client';

import { useState, useMemo, useEffect } from 'react';
import type { Team } from '@/lib/api/models/team';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { TeamContext } from '@/lib/contexts/team-context';
import { useUserInfo } from '@/hooks/use-user-info';

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
      // TODO: Update Team type to match v3 UserResponse or create proper conversion
      const userAsTeam: Team = {
        ...user,
        id: 0,
        created_at: 0,
        updated_at: 0,
        html_contents: user.description || '',
        user_type: user.user_type as any, // Cast to satisfy Team type
      };

      const userTeamsAsTeams: Team[] = (user.teams ?? []).map((team) => ({
        ...team,
        id: 0,
        created_at: 0,
        updated_at: 0,
        html_contents: '',
        user_type: team.user_type as any, // Cast to satisfy Team type
      }));

      setTeams([userAsTeam, ...userTeamsAsTeams]);
    }
  }, [user]);

  const selectedTeam = useMemo(() => {
    return teams[selectedIndex];
  }, [teams, selectedIndex]);

  const updateSelectedTeam = (updatedTeam: Team) => {
    const updatedTeams = teams.map((team) =>
      team.id === updatedTeam.id ? { ...team, ...updatedTeam } : team,
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
