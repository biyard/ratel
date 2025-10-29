import { useEffect, useMemo, useState, useCallback } from 'react';
import type { Attribute } from '../../types/answer-type';
import CustomCheckbox from '@/components/checkbox/custom-checkbox';
import type { TFunction } from 'i18next';

type AgePopupProps = {
  attributes: Attribute[];
  onSave: (attributes: Attribute[]) => void;
  onClose: () => void;
  t: TFunction<'SpacePanelEditor'>;
};

type AgeAttr =
  | { answer_type: 'age'; specific: number }
  | {
      answer_type: 'age';
      range: { inclusive_min: number; inclusive_max: number };
    };

type Preset = {
  key: string;
  label: string;
  make: () => AgeAttr;
  match: (a: AgeAttr) => boolean;
};

function buildPresets(t: TFunction<'SpacePanelEditor'>): Preset[] {
  return [
    {
      key: 'le17',
      label: t('age_under_17'),
      make: () => ({
        answer_type: 'age',
        range: { inclusive_min: 0, inclusive_max: 17 },
      }),
      match: (a) =>
        'range' in a &&
        a.range.inclusive_min === 0 &&
        a.range.inclusive_max === 17,
    },
    {
      key: '18-29',
      label: t('age_18_29'),
      make: () => ({
        answer_type: 'age',
        range: { inclusive_min: 18, inclusive_max: 29 },
      }),
      match: (a) =>
        'range' in a &&
        a.range.inclusive_min === 18 &&
        a.range.inclusive_max === 29,
    },
    {
      key: '30s',
      label: t('age_decade', { decade: 30 }),
      make: () => ({ answer_type: 'age', specific: 30 }),
      match: (a) => 'specific' in a && a.specific === 30,
    },
    {
      key: '40s',
      label: t('age_decade', { decade: 40 }),
      make: () => ({ answer_type: 'age', specific: 40 }),
      match: (a) => 'specific' in a && a.specific === 40,
    },
    {
      key: '50s',
      label: t('age_decade', { decade: 50 }),
      make: () => ({ answer_type: 'age', specific: 50 }),
      match: (a) => 'specific' in a && a.specific === 50,
    },
    {
      key: '60s',
      label: t('age_decade', { decade: 60 }),
      make: () => ({ answer_type: 'age', specific: 60 }),
      match: (a) => 'specific' in a && a.specific === 60,
    },
    {
      key: 'ge70',
      label: t('age_70_plus'),
      make: () => ({
        answer_type: 'age',
        range: { inclusive_min: 70, inclusive_max: 100 },
      }),
      match: (a) => 'range' in a && a.range.inclusive_min === 70,
    },
  ];
}

export default function AgePopup({
  attributes,
  onSave,
  onClose,
  t,
}: AgePopupProps) {
  const PRESETS = useMemo(() => buildPresets(t), [t]);

  const ageAttrs = useMemo(
    () => attributes.filter((x): x is AgeAttr => x.answer_type === 'age'),
    [attributes],
  );
  const otherAttrs = useMemo(
    () => attributes.filter((x) => x.answer_type !== 'age'),
    [attributes],
  );

  const [selected, setSelected] = useState<Set<string>>(new Set());

  useEffect(() => {
    const keys = new Set<string>();
    for (const a of ageAttrs)
      for (const p of PRESETS) if (p.match(a)) keys.add(p.key);
    setSelected(keys);
  }, [ageAttrs, PRESETS]);

  const toggle = useCallback((key: string) => {
    setSelected((prev) => {
      const next = new Set(prev);
      // eslint-disable-next-line @typescript-eslint/no-unused-expressions
      next.has(key) ? next.delete(key) : next.add(key);
      return next;
    });
  }, []);

  const handleSave = useCallback(() => {
    const nextAges = PRESETS.filter((p) => selected.has(p.key)).map((p) =>
      p.make(),
    );
    onSave([...otherAttrs, ...nextAges]);
    onClose();
  }, [PRESETS, selected, otherAttrs, onSave, onClose]);

  return (
    <div className="flex flex-col min-h-[300px] w-[500px] max-w-[500px] max-tablet:!w-full max-tablet:!max-w-full gap-5">
      <div className="max-h-[48vh] overflow-y-auto pr-2">
        <ul className="flex flex-col gap-3">
          {PRESETS.map((p) => {
            const id = `age-${p.key}`;
            return (
              <li key={p.key} className="flex items-center gap-3">
                <CustomCheckbox
                  checked={selected.has(p.key)}
                  onChange={() => toggle(p.key)}
                  disabled={false}
                />
                <label
                  htmlFor={id}
                  className="cursor-pointer text-[15px] text-text-primary select-none"
                >
                  {p.label}
                </label>
              </li>
            );
          })}
        </ul>
      </div>

      <div className="flex justify-end mt-2.5">
        <button
          className="w-fit px-10 py-[14.5px] rounded-[10px] bg-primary hover:bg-hover text-black font-bold text-base hover:text-black cursor-pointer"
          onClick={handleSave}
        >
          {t('save')}
        </button>
      </div>
    </div>
  );
}
