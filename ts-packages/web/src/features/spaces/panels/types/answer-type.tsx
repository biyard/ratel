import { PanelAttribute, PanelAttributeType } from './panel-attribute';

export type Attribute =
  | { answer_type: 'age'; specific: number }
  | {
      answer_type: 'age';
      range: { inclusive_min: number; inclusive_max: number };
    }
  | { answer_type: 'gender'; male: Record<string, never> }
  | { answer_type: 'gender'; female: Record<string, never> };

export const isAge = (
  a: Attribute,
): a is Extract<Attribute, { answer_type: 'age' }> => a.answer_type === 'age';

export const isAgeSpecific = (
  a: Attribute,
): a is Extract<Attribute, { answer_type: 'age'; specific: number }> =>
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  a.answer_type === 'age' && typeof (a as any).specific === 'number';

export const isAgeRange = (
  a: Attribute,
): a is Extract<
  Attribute,
  {
    answer_type: 'age';
    range: { inclusive_min: number; inclusive_max: number };
  }
> =>
  a.answer_type === 'age' &&
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  !!(a as any).range &&
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  typeof (a as any).range.inclusive_min === 'number' &&
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  typeof (a as any).range.inclusive_max === 'number';

export const isGender = (
  a: Attribute,
): a is Extract<Attribute, { answer_type: 'gender' }> =>
  a.answer_type === 'gender';

export function formatAgeLabel(a: Attribute): string | null {
  if (isAgeSpecific(a)) return `${a.specific}대`;
  if (isAgeRange(a)) {
    const { inclusive_min, inclusive_max } = a.range;
    if (inclusive_max === 17) return '17세 이하';
    if (inclusive_min === 70) return '70대 이상';
    return `${inclusive_min}–${inclusive_max}세`;
  }
  return null;
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function parsePanelAttribute(raw: any): PanelAttribute | null {
  if (raw == null) return null;

  if (typeof raw === 'object' && typeof raw.type === 'string') {
    const t = raw.type as string;
    const v =
      raw.value ??
      raw.verifiable_attribute ??
      raw.collective_attribute ??
      raw.VerifiableAttribute ??
      raw.CollectiveAttribute ??
      null;

    if (t === 'collective_attribute')
      return {
        type: PanelAttributeType.CollectiveAttribute,
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        value: String(v ?? 'none').toLowerCase() as any,
      };
    if (t === 'verifiable_attribute')
      return {
        type: PanelAttributeType.VerifiableAttribute,
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        value: v as any,
      };
  }

  if (typeof raw === 'string') {
    const [t, val = 'none'] = raw.split(':', 2);
    if (t === 'collective_attribute')
      return {
        type: PanelAttributeType.CollectiveAttribute,
        value: val.toLowerCase() as any,
      };
    if (t === 'verifiable_attribute')
      return {
        type: PanelAttributeType.VerifiableAttribute,
        value: val as any,
      };
  }

  return null;
}

export const parsePanelAttributes = (arr: unknown[]): PanelAttribute[] =>
  Array.isArray(arr)
    ? (arr.map(parsePanelAttribute).filter(Boolean) as PanelAttribute[])
    : [];
