import { BoosterType } from './booster-type';

export type SpacePublishState = 'Draft' | 'Published';

export type SpaceStatus = 'Waiting' | 'InProgress' | 'Finished';

export type SpaceVisibility =
  | { type: 'Private' }
  | { type: 'Public' }
  | { type: 'Team'; team_pk: string };

export interface SpaceCommon {
  pk: string;
  sk: string;

  created_at: number;
  updated_at: number;

  status?: SpaceStatus;
  publish_state: SpacePublishState;
  visibility: SpaceVisibility;
  post_pk: string;

  user_pk: string;
  author_display_name: string;
  author_profile_url: string;
  author_username: string;

  started_at?: number;
  ended_at?: number;

  booster: BoosterType;
  custom_booster?: number;
  rewards?: number;
}
