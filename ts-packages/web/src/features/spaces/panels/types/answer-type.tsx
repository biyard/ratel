export type Attribute =
  | { answer_type: 'age'; specific: number }
  | { answer_type: 'age'; inclusive_min: number; inclusive_max: number }
  | { answer_type: 'gender'; value: 'male' | 'female' };

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function parseAttribute(raw: any): Attribute | null {
  if (!raw || typeof raw !== 'object') return null;

  const tag = raw.answer_type as 'age' | 'gender' | undefined;

  if (tag === 'age') {
    if (typeof raw.specific === 'number') {
      return { answer_type: 'age', specific: raw.specific };
    }
    const range = raw.range && typeof raw.range === 'object' ? raw.range : raw;
    if (
      typeof range.inclusive_min === 'number' &&
      typeof range.inclusive_max === 'number'
    ) {
      return {
        answer_type: 'age',
        inclusive_min: range.inclusive_min,
        inclusive_max: range.inclusive_max,
      };
    }
    return null;
  }

  if (tag === 'gender') {
    const cand =
      raw.value ??
      raw.gender ??
      raw.Gender ??
      raw.gender_value ??
      raw.val ??
      raw.g;

    if (typeof cand === 'string') {
      const s = cand.toLowerCase();
      if (s === 'male' || s === 'female')
        return { answer_type: 'gender', value: s };
    }
    if (typeof cand === 'number') {
      if (cand === 1) return { answer_type: 'gender', value: 'male' };
      if (cand === 2) return { answer_type: 'gender', value: 'female' };
    }
    if (typeof raw === 'string') {
      const s = raw.toLowerCase();
      if (s === 'male' || s === 'female')
        return { answer_type: 'gender', value: s };
    }
    return null;
  }

  return null;
}
