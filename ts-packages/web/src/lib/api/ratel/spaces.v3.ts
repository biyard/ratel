import { BoosterType } from '@/types/booster-type';
import { SpaceType } from '@/types/space-type';
import { call } from './call';

export type CreateSpaceResponse = {
  space_pk: string;
};

export function createSpace(
  postPk: string,
  spaceType: SpaceType,
  startedAt: number | null,
  endedAt: number | null,
  booster: BoosterType | null,
): Promise<CreateSpaceResponse> {
  return call('POST', '/v3/spaces', {
    space_type: spaceType,
    post_pk: postPk,
    started_at: startedAt,
    ended_at: endedAt,
    booster: booster,
  });
}
