import { BoosterType } from '../features/spaces/types/booster-type';

export enum SpacePublishState {
  Draft = 'Draft',
  Published = 'Published',
}

export enum SpaceStatus {
  Waiting = 'Waiting',
  InProgress = 'InProgress',
  Finished = 'Finished',
}

export type SpaceVisibility =
  | { type: 'Private' }
  | { type: 'Public' }
  | { type: 'Team'; team_pk: string };

export const SpaceVisibilityValue = {
  Private: { type: 'Private' } as const,
  Public: { type: 'Public' } as const,
  Team: (team_pk: string) => ({ type: 'Team', team_pk }) as const,
};

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
