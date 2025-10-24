import { BoosterType } from './booster-type';

export enum SpacePublishState {
  Draft = 'DRAFT',
  Published = 'PUBLISHED',
}

export enum SpaceStatus {
  Waiting = 'WAITING',
  InProgress = 'IN_PROGRESS',
  Finished = 'FINISHED',
}

export type SpaceVisibility =
  | { type: 'private' }
  | { type: 'public' }
  | { type: 'team'; team_pk: string };

export function normalizeVisibility(v: unknown): SpaceVisibility | undefined {
  const type = v.toString().toLowerCase();

  if (type === 'private') return { type: 'private' };
  if (type === 'public') return { type: 'public' };
  if (type === 'team') {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const team_pk = (v as any).team_pk;
    if (typeof team_pk === 'string' && team_pk) {
      return { type: 'team', team_pk };
    }
  }
  return undefined;
}

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
