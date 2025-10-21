import CalendarDropdown from '@/components/calendar-dropdown';
import { Internet } from '@/components/icons';
import TimeDropdown from '@/components/time-dropdown';
import { Button } from '@/components/ui/button';
import { cn } from '@/lib/utils';
import { useState } from 'react';
import { useTranslation } from 'react-i18next';

export type TimeRangeSettingProps = {
  startTimestampMillis: number;
  endTimestampMillis: number;
  onChange: (startedAt: number, endedAt: number) => void;
  canEdit?: boolean;
  className?: string;
};

export const i18nTimeRangeSetting = {
  en: {
    btn_edit: 'Change',
    btn_save: 'Save',
  },
  ko: {
    btn_edit: '수정',
    btn_save: '저장',
  },
};

export function TimeRangeSetting({
  onChange,
  startTimestampMillis,
  endTimestampMillis,
  className,
  canEdit,
}: TimeRangeSettingProps) {
  const [start, setStart] = useState(startTimestampMillis);
  const [end, setEnd] = useState(endTimestampMillis);
  const [delta, setDelta] = useState(end - start);
  const localTimezone = Intl.DateTimeFormat().resolvedOptions().timeZone;
  const { t } = useTranslation('TimeRangeSetting');
  const [editing, setEditing] = useState(false);

  const handleStart = (ts: number) => {
    setStart(ts);
    setEnd(ts + delta);
  };

  const handleEnd = (ts: number) => {
    setEnd(ts);
    setDelta(ts - start);
  };

  let button = <></>;
  if (canEdit) {
    button = editing ? (
      <Button
        variant="primary"
        onClick={() => {
          setEditing(false);
          onChange(start, end);
        }}
      >
        {' '}
        {t('btn_save')}
      </Button>
    ) : (
      <Button
        onClick={() => {
          setEditing(true);
        }}
      >
        {' '}
        {t('btn_edit')}
      </Button>
    );
  }

  return (
    <>
      <div
        className={cn(
          'flex flex-row gap-2 w-full max-tablet:flex-wrap max-tablet:items-center',
          className,
        )}
      >
        <div className="flex flex-row gap-2 w-full max-tablet:flex-wrap max-tablet:items-center">
          <div className="flex flex-col gap-2 w-full sm:flex-row sm:items-center sm:w-auto">
            <div className="w-full max-tablet:w-auto">
              <CalendarDropdown
                canEdit={canEdit && editing}
                value={start}
                onChange={handleStart}
              />
            </div>
            <div className="w-full sm:w-auto">
              <TimeDropdown
                canEdit={canEdit && editing}
                value={start}
                onChange={handleStart}
              />
            </div>
          </div>

          <div className="hidden self-center h-0.5 sm:block w-[15px] bg-neutral-600" />

          <div className="flex flex-col gap-2 w-full sm:flex-row sm:items-center sm:w-auto max-tablet:mt-2.5">
            <div className="w-full sm:w-auto">
              <CalendarDropdown
                canEdit={canEdit && editing}
                value={end}
                onChange={handleEnd}
              />
            </div>
            <div className="w-full sm:w-auto">
              <TimeDropdown
                canEdit={canEdit && editing}
                value={end}
                onChange={handleEnd}
              />
            </div>
          </div>

          <div className="flex flex-row gap-2.5 items-center px-5 mt-2 w-full rounded-lg border sm:mt-0 border-select-date-border bg-select-date-bg py-[10.5px] sm:w-fit">
            <div className="font-medium text-[15px]/[22.5px] text-neutral-600">
              {localTimezone}
            </div>
            <Internet
              className="w-5 h-5 [&>path]:stroke-neutral-600 [&>circle]:stroke-neutral-600"
              width="20"
              height="20"
            />
          </div>
        </div>

        {button}
      </div>
    </>
  );
}
