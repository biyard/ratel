import { UserType } from './user';

export interface NetworkData {
  industries: Industry[];
  suggested_teams: Follower[];
  suggested_users: Follower[];
}

export interface Industry {
  id: number;
  created_at: number;
  updated_at: number;

  name: string;
}

export interface Follower {
  id: number;
  created_at: number;
  updated_at: number;

  user_type: UserType;
  nickname: string;
  profile_url: string;
  email: string;
  username: string;
  html_contents: string;

  followers_count: number;
  followings_count: number;

  followers: Follower[];
  followings: Follower[];
}
