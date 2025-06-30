import { createContext } from 'react';
import { Team } from '@/lib/api/models/team';

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
