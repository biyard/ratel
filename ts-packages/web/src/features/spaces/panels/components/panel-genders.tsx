import React from 'react';
import type { Attribute } from '../types/answer-type';
import { TFunction } from 'i18next';

type PanelGenderProps = {
  attributes: Attribute[];
  t: TFunction<'SpacePanelEditor'>;
};

export function PanelGender({ t, attributes }: PanelGenderProps) {
  const genders = attributes.filter((a) => a.answer_type === 'gender');

  if (genders.length === 0) {
    return <span className="text-neutral-500">—</span>;
  }

  return (
    <div className="flex flex-wrap gap-2">
      {genders.map((g, i) => (
        <span
          key={i}
          className="inline-flex items-center rounded-full bg-neutral-800/70 px-2.5 py-0.5 text-xs font-medium text-neutral-200 ring-1 ring-neutral-700"
        >
          {'male' in g ? t('male') : 'female' in g ? t('female') : '—'}
        </span>
      ))}
    </div>
  );
}
