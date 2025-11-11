import FileModel from '../files/types/file';
import { BoosterType } from './booster-type';

export enum SpacePublishState {
  Draft = 'DRAFT',
  Published = 'PUBLISHED',
}

export enum SpaceStatus {
  Waiting = 'WAITING',
  InProgress = 'IN_PROGRESS',
  Started = 'STARTED',
  Finished = 'FINISHED',
}

export type SpaceVisibility =
  | { type: 'Private' | 'private' | 'PRIVATE' }
  | { type: 'Public' | 'public' | 'PUBLIC' }
  | { type: 'Team' | 'team' | 'TEAM'; team_pk: string };

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
  files?: FileModel[];

  anonymous_participation: boolean;
}

export type MySpace = SpaceCommon & {
  title: string;
  invitation_status: 'pending' | 'participating';
};
