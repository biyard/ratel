import { Attribute } from '@/features/spaces/panels/types/answer-type';
import { call } from './call';
import { PanelAttribute } from '@/features/spaces/panels/types/panel-attribute';

export function createSpacePanel(
  spacePk: string,

  quotas: number,
  attributes: Attribute[],
): Promise<void> {
  return call('POST', `/v3/spaces/${encodeURIComponent(spacePk)}/panels`, {
    name,
    quotas,
    attributes,
  });
}

export function createSpacePanelQuotas(
  spacePk: string,

  quotas: number[],
  attributes: PanelAttribute[],
): Promise<void> {
  return call(
    'POST',
    `/v3/spaces/${encodeURIComponent(spacePk)}/panels/quotas`,
    {
      quotas,
      attributes,
    },
  );
}

export function updateSpacePanelQuotas(
  spacePk: string,

  quotas: number,
  attribute: PanelAttribute,
): Promise<void> {
  return call(
    'PATCH',
    `/v3/spaces/${encodeURIComponent(spacePk)}/panels/quotas`,
    {
      quotas,
      attribute,
    },
  );
}

export function deleteSpacePanelQuotas(
  spacePk: string,

  attribute: PanelAttribute,
): Promise<void> {
  return call(
    'DELETE',
    `/v3/spaces/${encodeURIComponent(spacePk)}/panels/quotas`,
    {
      attribute,
    },
  );
}

export function updateSpacePanel(
  spacePk: string,

  quotas: number,
  attributes: PanelAttribute[],
): Promise<void> {
  return call('PATCH', `/v3/spaces/${encodeURIComponent(spacePk)}/panels`, {
    quotas,
    attributes,
  });
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

export function participateSpacePanel(
  spacePk: string,
  panelPk: string,
): Promise<void> {
  return call(
    'PATCH',
    `/v3/spaces/${encodeURIComponent(spacePk)}/panels/${encodeURIComponent(panelPk)}/participants`,
    {},
  );
}
