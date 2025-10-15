import { BoosterType } from '../features/spaces/types/booster-type';

/**
 * @deprecated import { SpacePublishState } from '@/features/spaces/types/space-common';
 * Use `SpacePublishState` from `features/spaces/types/space-common` instead.
 */
export enum SpacePublishState {
  Draft = 'DRAFT',
  Published = 'PUBLISHED',
}

/**
 * @deprecated import { SpaceStatus } from '@/features/spaces/types/space-common';
 * Use `SpaceStatus` from `features/spaces/types/space-common` instead.
 */

export enum SpaceStatus {
  Waiting = 'WAITING',
  InProgress = 'IN_PROGRESS',
  Finished = 'FINISHED',
}

/**
 * @deprecated import { SpaceVisibility } from '@/features/spaces/types/space-common';
 * Use `SpaceVisibility` from `features/spaces/types/space-common` instead.
 */
export type SpaceVisibility =
  | { type: 'Private' }
  | { type: 'Public' }
  | { type: 'Team'; team_pk: string };

/**
 * @deprecated
 */
export const SpaceVisibilityValue = {
  Private: { type: 'Private' } as const,
  Public: { type: 'Public' } as const,
  Team: (team_pk: string) => ({ type: 'Team', team_pk }) as const,
};

/**
 * @deprecated import { SpaceCommon } from '@/features/spaces/types/space-common';
 * Use `SpaceCommon` from `features/spaces/types/space-common` instead.
 */
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
