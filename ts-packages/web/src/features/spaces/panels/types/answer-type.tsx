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
export function parseAttribute(raw: any): Attribute | null {
  if (!raw || typeof raw !== 'object') return null;

  if (raw.answer_type === 'age') {
    if (typeof raw.specific === 'number') {
      return { answer_type: 'age', specific: raw.specific };
    }
    if (
      raw.range &&
      typeof raw.range.inclusive_min === 'number' &&
      typeof raw.range.inclusive_max === 'number'
    ) {
      return {
        answer_type: 'age',
        range: {
          inclusive_min: raw.range.inclusive_min,
          inclusive_max: raw.range.inclusive_max,
        },
      };
    }
    return null;
  }

  if (raw.answer_type === 'gender') {
    if ('male' in raw) return { answer_type: 'gender', male: {} };
    if ('female' in raw) return { answer_type: 'gender', female: {} };
    return null;
  }

  return null;
}

export const parseAttributes = (arr: unknown[]): Attribute[] =>
  Array.isArray(arr)
    ? (arr.map(parseAttribute).filter(Boolean) as Attribute[])
    : [];
