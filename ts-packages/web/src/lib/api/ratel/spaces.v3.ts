import { SpaceType } from '@/features/spaces/types/space-type';
import { call } from './call';
import { SpaceVisibility } from '@/features/spaces/types/space-common';

export type CreateSpaceResponse = {
  space_pk: string;
};

export function getSpaceByPostPk(postPk: string): Promise<unknown> {
  return call('GET', `/v3/spaces/${encodeURIComponent(postPk)}`);
}

export function createSpace(
  postPk: string,
  spaceType: SpaceType,
): Promise<CreateSpaceResponse> {
  return call('POST', '/v3/spaces', {
    space_type: spaceType,
    post_pk: postPk,
  });
}

function encodeVisibility(visibility: SpaceVisibility) {
  if (visibility.type === 'Team') {
    return `TEAM#${visibility.team_pk}`;
  }
  return visibility.type.toUpperCase();
}
export function publishSpace(
  spacePk: string,
  visibility: SpaceVisibility,
): Promise<void> {
  return call('PATCH', `/v3/spaces/${encodeURIComponent(spacePk)}`, {
    publish: true,
    visibility: encodeVisibility(visibility),
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

export function updateSpaceContent(
  spacePk: string,
  content: string,
): Promise<void> {
  return call('PATCH', `/v3/spaces/${encodeURIComponent(spacePk)}`, {
    content,
  });
}

export function updateSpaceTitle(
  spacePk: string,
  title: string,
): Promise<void> {
  return call('PATCH', `/v3/spaces/${encodeURIComponent(spacePk)}`, {
    title,
  });
}
