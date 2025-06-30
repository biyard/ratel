'use client';

import React, { useState, useMemo, useEffect } from 'react';
import { Team } from '@/lib/api/models/team';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { TeamContext } from '@/lib/contexts/team-context';

export const TeamProvider = ({ children }: { children: React.ReactNode }) => {
  const { data: user } = useSuspenseUserInfo();
  const [selectedIndex, setSelectedTeam] = useState(0);
  const [teams, setTeams] = useState<Team[]>([]);

  useEffect(() => {
    if (user) {
      setTeams([{ ...user }, ...(user.teams ?? [])]);
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
