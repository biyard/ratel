import { useEffect, useMemo, useState, useCallback } from 'react';
import type { Attribute } from '../../types/answer-type';
import CustomCheckbox from '@/components/checkbox/custom-checkbox';
import type { TFunction } from 'i18next';

type GenderPopupProps = {
  attributes: Attribute[];
  onSave: (attributes: Attribute[]) => void;
  onClose: () => void;
  t: TFunction<'SpacePanelEditor'>;
};

type GenderPreset = {
  key: 'male' | 'female';
  label: string;
  make: () => Attribute;
  match: (a: Attribute) => boolean;
};

export default function GenderPopup({
  attributes,
  onSave,
  onClose,
  t,
}: GenderPopupProps) {
  const PRESETS: GenderPreset[] = useMemo(
    () => [
      {
        key: 'male',
        label: t('male'),
        make: () => ({ answer_type: 'gender', male: {} }),
        match: (a) => a.answer_type === 'gender' && 'male' in a,
      },
      {
        key: 'female',
        label: t('female'),
        make: () => ({ answer_type: 'gender', female: {} }),
        match: (a) => a.answer_type === 'gender' && 'female' in a,
      },
    ],
    [t],
  );

  const genderAttrs = useMemo(
    () => attributes.filter((x) => x.answer_type === 'gender'),
    [attributes],
  );
  const otherAttrs = useMemo(
    () => attributes.filter((x) => x.answer_type !== 'gender'),
    [attributes],
  );

  const [selected, setSelected] = useState<Set<string>>(new Set());

  useEffect(() => {
    const keys = new Set<string>();
    for (const a of genderAttrs)
      for (const p of PRESETS) if (p.match(a)) keys.add(p.key);
    setSelected(keys);
  }, [genderAttrs, PRESETS]);

  const toggle = useCallback((key: string) => {
    setSelected((prev) => {
      const next = new Set(prev);
      // eslint-disable-next-line @typescript-eslint/no-unused-expressions
      next.has(key) ? next.delete(key) : next.add(key);
      return next;
    });
  }, []);

  const handleSave = useCallback(() => {
    const nextGenders = PRESETS.filter((p) => selected.has(p.key)).map((p) =>
      p.make(),
    );
    onSave([...otherAttrs, ...nextGenders]);
    onClose();
  }, [PRESETS, selected, otherAttrs, onSave, onClose]);

  return (
    <div className="flex flex-col min-h-[150px] w-[400px] max-w-[400px] gap-5">
      <div className="max-h-[40vh] overflow-y-auto pr-2">
        <ul className="flex flex-col gap-3">
          {PRESETS.map((p) => {
            const id = `gender-${p.key}`;
            return (
              <li key={p.key} className="flex items-center gap-3">
                <CustomCheckbox
                  checked={selected.has(p.key)}
                  onChange={() => toggle(p.key)}
                  disabled={false}
                />
                <label
                  onClick={() => toggle(p.key)}
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
