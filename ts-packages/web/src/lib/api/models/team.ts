import { Group, User, UserType } from './user';

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
  email: string;
  profile_url?: string;
  user_type: UserType;

  parent_id?: number;
  username: string;

  html_contents: string;
  groups?: Group[];
  members?: User[];
}
