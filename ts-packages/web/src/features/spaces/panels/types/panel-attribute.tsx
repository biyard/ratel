export type PanelAttribute =
  | { type: 'none' }
  | {
      type: 'collective_attribute';
      value: 'none' | 'university' | 'age' | 'gender';
    }
  | { type: 'verifiable_attribute'; value: 'none' | 'age' | 'gender' };

export type Attribute =
  | { answer_type: 'age'; specific: number }
  | {
      answer_type: 'age';
      range: { inclusive_min: number; inclusive_max: number };
    }
  | { answer_type: 'gender'; male: Record<string, never> }
  | { answer_type: 'gender'; female: Record<string, never> };

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export function parsePanelAttribute(input: any): PanelAttribute | null {
  if (!input) return null;

  if (typeof input === 'object' && typeof input.type === 'string') {
    const t = input.type as PanelAttribute['type'];

    const v =
      input.value ??
      input.verifiable_attribute ??
      input.collective_attribute ??
      input.CollectiveAttribute ??
      input.VerifiableAttribute ??
      null;

    if (t === 'none') return { type: 'none' };
    if (t === 'collective_attribute') {
      const vv = (typeof v === 'string' ? v : 'none').toLowerCase();
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      return { type: 'collective_attribute', value: vv as any };
    }
    if (t === 'verifiable_attribute') {
      const vv = (typeof v === 'string' ? v : 'none').toLowerCase();
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      return { type: 'verifiable_attribute', value: vv as any };
    }
  }

  if (typeof input === 'string') {
    if (input === 'none') return { type: 'none' };
    const [t, raw] = input.split(':', 2);
    if (t === 'collective_attribute') {
      return {
        type: 'collective_attribute',
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        value: (raw ?? 'none').toLowerCase() as any,
      };
    }
    if (t === 'verifiable_attribute') {
      return {
        type: 'verifiable_attribute',
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        value: (raw ?? 'none').toLowerCase() as any,
      };
    }
  }

  return null;
}
