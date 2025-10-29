import { Attribute } from '@/features/spaces/panels/types/answer-type';
import { call } from './call';

export function createSpacePanel(
  spacePk: string,

  name: string,
  quotas: number,
  attributes: Attribute[],
): Promise<void> {
  return call('POST', `/v3/spaces/${encodeURIComponent(spacePk)}/panels`, {
    name,
    quotas,
    attributes,
  });
}

export function updateSpacePanel(
  spacePk: string,
  panelPk: string,

  name: string,
  quotas: number,
  attributes: Attribute[],
): Promise<void> {
  return call(
    'PATCH',
    `/v3/spaces/${encodeURIComponent(spacePk)}/panels/${encodeURIComponent(panelPk)}`,
    {
      name,
      quotas,
      attributes,
    },
  );
}

export function deleteSpacePanel(
  spacePk: string,
  panelPk: string,
): Promise<void> {
  return call(
    'DELETE',
    `/v3/spaces/${encodeURIComponent(spacePk)}/panels/${encodeURIComponent(panelPk)}`,
    {},
  );
}
