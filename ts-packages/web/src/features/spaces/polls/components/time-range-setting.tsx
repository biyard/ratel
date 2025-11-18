import CalendarDropdown from '@/components/calendar-dropdown';
import TimeDropdown from '@/components/time-dropdown';
import TimezoneDropdown from '@/components/timezone-dropdown';
import { Button } from '@/components/ui/button';
import { cn } from '@/lib/utils';
import { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';

export type TimeRangeSettingProps = {
  startTimestampMillis: number;
  endTimestampMillis: number;
  onChange?: (startedAt: number, endedAt: number) => void;
  canEdit?: boolean;
  alwaysEdit?: boolean;
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
  alwaysEdit = false,
  canEdit,
}: TimeRangeSettingProps) {
  const [start, setStart] = useState(startTimestampMillis);
  const [end, setEnd] = useState(endTimestampMillis);
  const [delta, setDelta] = useState(end - start);
  const localTimezone = Intl.DateTimeFormat().resolvedOptions().timeZone;
  const [timezone, setTimezone] = useState(localTimezone);
  const { t } = useTranslation('TimeRangeSetting');
  const [editing, setEditing] = useState(false);

  useEffect(() => {
    setStart(startTimestampMillis);
    setEnd(endTimestampMillis);
    setDelta(endTimestampMillis - startTimestampMillis);
  }, [startTimestampMillis, endTimestampMillis]);

  const handleStart = (ts: number) => {
    setStart(ts);
    setEnd(ts + delta);

    if (alwaysEdit) {
      onChange(ts, ts + delta);
    }
  };

  const handleEnd = (ts: number) => {
    setEnd(ts);
    setDelta(ts - start);

    if (alwaysEdit) {
      onChange(start, ts);
    }
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

  const isEdit = alwaysEdit || (canEdit && editing);

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
                canEdit={isEdit}
                value={start}
                onChange={handleStart}
                data-testid="calendar-start-date-dropdown"
              />
            </div>
            <div className="w-full sm:w-auto">
              <TimeDropdown
                canEdit={isEdit}
                value={start}
                onChange={handleStart}
                data-testid="time-start-dropdown"
              />
            </div>
          </div>

          <div className="hidden self-center h-0.5 sm:block w-[15px] bg-neutral-600" />

          <div className="flex flex-col gap-2 w-full sm:flex-row sm:items-center sm:w-auto max-tablet:mt-2.5">
            <div className="w-full sm:w-auto">
              <CalendarDropdown
                canEdit={isEdit}
                value={end}
                onChange={handleEnd}
                data-testid="calendar-end-date-dropdown"
              />
            </div>
            <div className="w-full sm:w-auto">
              <TimeDropdown
                canEdit={isEdit}
                value={end}
                onChange={handleEnd}
                data-testid="time-end-dropdown"
              />
            </div>
          </div>

          <TimezoneDropdown
            value={timezone}
            onChange={setTimezone}
            canEdit={isEdit}
          />
        </div>

        {button}
      </div>
    </>
  );
}
