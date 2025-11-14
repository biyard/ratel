import { useEffect, useMemo } from 'react';
import { TFunction } from 'i18next';
import { SurveySummary } from '@/features/spaces/polls/types/poll-question';

export type Primary = 'overall' | 'gender' | 'age' | 'school';

type Props = {
  t: TFunction<'SpaceSurveyReport', undefined>;
  primary: Primary;
  setPrimary: (p: Primary) => void;
  detailKey: string | null;
  setDetailKey: (k: string | null) => void;
  summariesByGender?: Record<string, SurveySummary[]>;
  summariesByAge?: Record<string, SurveySummary[]>;
  summariesBySchool?: Record<string, SurveySummary[]>;
};

export default function SummaryOption({
  t,
  primary,
  setPrimary,
  detailKey,
  setDetailKey,
  summariesByGender,
  summariesByAge,
  summariesBySchool,
}: Props) {
  const detailOptions = useMemo(() => {
    switch (primary) {
      case 'gender':
        return Object.keys(summariesByGender ?? {});
      case 'age':
        return Object.keys(summariesByAge ?? {});
      case 'school':
        return Object.keys(summariesBySchool ?? {});
      default:
        return [];
    }
  }, [primary, summariesByGender, summariesByAge, summariesBySchool]);

  useEffect(() => {
    if (primary === 'overall') setDetailKey(null);
    else setDetailKey(detailOptions[0] ?? null);
  }, [primary, detailOptions, setDetailKey]);

  return (
    <div className="flex items-center gap-3 mb-4 mt-5">
      <select
        className="px-3 py-2 h-9 rounded border border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
        value={primary}
        onChange={(e) => setPrimary(e.target.value as Primary)}
      >
        <option value="overall">{t('overall')}</option>
        {summariesByGender && Object.keys(summariesByGender).length > 0 && (
          <option value="gender">{t('gender')}</option>
        )}
        {summariesByAge && Object.keys(summariesByAge).length > 0 && (
          <option value="age">{t('age')}</option>
        )}
        {summariesBySchool && Object.keys(summariesBySchool).length > 0 && (
          <option value="school">{t('school')}</option>
        )}
      </select>

      {primary !== 'overall' && detailOptions.length > 0 && (
        <select
          className="px-3 py-2 h-9 rounded border border-gray-300 dark:border-gray-600 dark:bg-gray-700 dark:text-white"
          value={detailKey ?? ''}
          onChange={(e) => setDetailKey(e.target.value)}
        >
          {detailOptions.map((k) => (
            <option key={k} value={k}>
              {k}
            </option>
          ))}
        </select>
      )}
    </div>
  );
}
