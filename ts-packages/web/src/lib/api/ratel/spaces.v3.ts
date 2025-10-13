import { BoosterType } from '@/types/booster-type';
import { SpaceType } from '@/types/space-type';
import { call } from './call';
import { SpaceVisibility } from '@/types/space-common';

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

export function publishSpace(
  spacePk: string,
  visibility: SpaceVisibility,
): Promise<void> {
  return call('PATCH', `/v3/spaces/${encodeURIComponent(spacePk)}`, {
    publish: true,
    visibility,
  });
}

export function updateSpaceVisibility(
  spacePk: string,
  visibility: SpaceVisibility,
): Promise<void> {
  return call('PATCH', `/v3/spaces/${encodeURIComponent(spacePk)}`, {
    visibility,
  });
}
