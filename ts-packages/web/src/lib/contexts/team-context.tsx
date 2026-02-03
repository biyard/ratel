import { createContext, useContext } from 'react';
import type { Team } from '@/features/teams/types/team';

export interface TeamContextType {
  teams: Team[];
  selectedTeam: Team;
  selectedIndex: number;
  setSelectedTeam: (index: number) => void;
  updateSelectedTeam: (team: Team) => void;
}

export const TeamContext = createContext<TeamContextType>({
  teams: [],
  selectedTeam: {} as Team,
  selectedIndex: 0,
  setSelectedTeam: () => {},
  updateSelectedTeam: () => {},
});

export function useTeamContext() {
  const context = useContext(TeamContext);
  if (!context) {
    throw new Error('useTeamContext must be used within a TeamProvider');
  }
  return context;
}
