import React from 'react';
import type { Attribute } from '../types/answer-type';
import { TFunction } from 'i18next';

type AgeAttr =
  | { answer_type: 'age'; specific: number }
  | {
      answer_type: 'age';
      range: { inclusive_min: number; inclusive_max: number };
    };

function isAge(a: Attribute): a is AgeAttr {
  return a.answer_type === 'age';
}

function formatAgeLabel(a: AgeAttr, t: TFunction<'SpacePanelEditor'>) {
  if ('specific' in a) return t('age_decade', { decade: a.specific });

  const { inclusive_min, inclusive_max } = a.range;
  if (inclusive_max === 17) return t('age_under_17');
  if (inclusive_min === 18) return t('age_18_29');
  if (inclusive_min === 70) return t('age_70_plus');
  return t('age_decade', { decade: inclusive_min });
}

type PanelAgeProps = {
  attributes: Attribute[];
  t: TFunction<'SpacePanelEditor'>;
};

export function PanelAge({ attributes, t }: PanelAgeProps) {
  const ages = attributes.filter(isAge);

  if (ages.length === 0) {
    return <span className="text-neutral-500">—</span>;
  }

  return (
    <div className="flex flex-wrap gap-2">
      {ages.map((a, i) => (
        <span
          key={i}
          className="inline-flex items-center rounded-full bg-neutral-800/70 px-2.5 py-0.5 text-xs font-medium text-neutral-200 ring-1 ring-neutral-700"
        >
          {formatAgeLabel(a, t)}
        </span>
      ))}
    </div>
  );
}
