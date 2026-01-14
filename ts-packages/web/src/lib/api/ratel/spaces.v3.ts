import { SpaceType } from '@/features/spaces/types/space-type';
import { call } from './call';
import { SpaceVisibility } from '@/features/spaces/types/space-common';
import FileModel from '@/features/spaces/files/types/file';

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

export function deleteSpace(spacePk: string): Promise<void> {
  return call('DELETE', `/v3/spaces/${encodeURIComponent(spacePk)}`);
}

export function updateSpaceVisibility(
  spacePk: string,
  visibility: SpaceVisibility,
): Promise<void> {
  return call('PATCH', `/v3/spaces/${encodeURIComponent(spacePk)}`, {
    visibility,
  });
}

export function updateSpaceFiles(
  spacePk: string,
  files: FileModel[],
): Promise<{ files: FileModel[] }> {
  return call('PATCH', `/v3/spaces/${encodeURIComponent(spacePk)}`, {
    files,
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

export function updateSpaceAnonymousParticipation(
  spacePk: string,
  anonymousParticipation: boolean,
): Promise<void> {
  return call('PATCH', `/v3/spaces/${encodeURIComponent(spacePk)}`, {
    anonymous_participation: anonymousParticipation,
  });
}

export function updateSpaceChangeVisibility(
  spacePk: string,
  changeVisibility: boolean,
): Promise<void> {
  return call('PATCH', `/v3/spaces/${encodeURIComponent(spacePk)}`, {
    change_visibility: changeVisibility,
  });
}

export type ParticipateSpaceRequest = {
  verifiable_presentation: string;
};

export type ParticipateSpaceResponse = {
  username: string;
  display_name: string;
  profile_url: string;
};

export function participateSpace(
  spacePk: string,
  verifiablePresentation: string,
): Promise<ParticipateSpaceResponse> {
  return call('POST', `/v3/spaces/${encodeURIComponent(spacePk)}/participate`, {
    verifiable_presentation: verifiablePresentation,
  });
}
