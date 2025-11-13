import {
  CollectiveAttribute,
  Gender,
  PanelAttribute,
  PanelAttributeType,
  PanelAttributeWithQuota,
  VerifiableAttribute,
  VerifiableAttributeType,
  VerifiableAttributeWithQuota,
} from '../../types/panel-attribute';

export enum PanelAttributeOptions {
  University = 'university',
  Gender = 'gender',
}

export function getAllPanelAttributeOptions() {
  return [PanelAttributeOptions.Gender, PanelAttributeOptions.University];
}

export function convertOptionsToPanelAttributes(
  options: PanelAttributeOptions[],
): PanelAttribute[] {
  const ret = [];

  for (const opt of options) {
    switch (opt) {
      case PanelAttributeOptions.University:
        ret.push({
          type: PanelAttributeType.CollectiveAttribute,
          value: CollectiveAttribute.University,
        });
        break;

      case PanelAttributeOptions.Gender:
        ret.push({
          type: PanelAttributeType.VerifiableAttribute,
          value: {
            type: VerifiableAttributeType.Gender,
            value: Gender.Male,
          },
        });

        ret.push({
          type: PanelAttributeType.VerifiableAttribute,
          value: {
            type: VerifiableAttributeType.Gender,
            value: Gender.Female,
          },
        });
    }
  }

  return ret;
}

export function getAttributeWithDefaultQuotas(
  totalQuota: number,
  attributes: PanelAttribute[],
): PanelAttributeWithQuota[] {
  const numOfGender = attributes.filter(
    (attr) =>
      attr.type === PanelAttributeType.VerifiableAttribute &&
      (attr.value as VerifiableAttribute).type ===
        VerifiableAttributeType.Gender,
  );

  let divisor = numOfGender.length || 1;
  divisor = divisor === 0 ? 1 : divisor;
  const defaultQuota = Math.ceil(totalQuota / divisor);

  return attributes.map((e) => {
    if (e.type === PanelAttributeType.VerifiableAttribute) {
      const verAttr = e.value as VerifiableAttribute;

      return {
        ...e,

        value: {
          ...verAttr,
          quota: defaultQuota,
        } as VerifiableAttributeWithQuota,
      } as PanelAttributeWithQuota;
    }

    return e as PanelAttributeWithQuota;
  });
}
