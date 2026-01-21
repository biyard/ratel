import { User, UserType } from '../ratel/users.v3';
import type { Group } from './user';

export interface DeleteTeamRequest {
  team_id: number;
}

export function deleteTeamRequest(team_id: number): DeleteTeamRequest {
  return {
    team_id: team_id,
  };
}

export interface CreateTeamRequest {
  create: {
    profile_url: string;
    username: string;
    nickname: string;
    html_contents: string;
  };
}

export function createTeamRequest(
  profile_url: string,
  username: string,
  nickname: string,
  html_contents: string,
): CreateTeamRequest {
  return {
    create: {
      profile_url,
      username,
      nickname,
      html_contents,
    },
  };
}

export interface Team {
  id: number;
  created_at: number;
  updated_at: number;

  nickname: string;
  profile_url?: string;
  dao_address?: string;
  user_type: UserType;

  parent_id?: number;
  username: string;

  html_contents: string;
  groups?: Group[];
  members?: User[];
}
