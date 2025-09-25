'use client';

import React, { useState } from 'react';
// import CustomCheckbox from '@/components/checkbox/custom-checkbox';
import { Internet } from '@/components/icons';
import { showErrorToast } from '@/lib/toast';
import TimeDropdown from '@/components/time-dropdown';
import CalendarDropdown from '@/components/calendar-dropdown';
import { useTranslations } from 'next-intl';

export default function SetSchedulePopup({
  startedAt,
  endedAt,
  onconfirm,
}: {
  startedAt: number;
  endedAt: number;
  onconfirm: (startDate: number, endDate: number) => void;
}) {
  const t = useTranslations('PollSpace');
  // const [autoSchedule, setAutoSchedule] = useState(true);

  // const initialDate = Math.floor(Date.now());

  const [sessions, setSessions] = useState([
    { label: t('poll'), start: startedAt * 1000, end: endedAt * 1000 },
  ]);

  const handleChange = (index: number, key: 'start' | 'end', value: number) => {
    const updated = [...sessions];
    updated[index][key] = value;
    setSessions(updated);
  };

  const handleConfirm = () => {
    for (const s of sessions) {
      if (s.start >= s.end) {
        showErrorToast(t('start_date_warning'));
        return;
      }
    }

    onconfirm(sessions[0].start, sessions[0].end);
  };

  return (
    <div className="w-full max-w-[900px] p-8 rounded-2xl">
      <div className="text-2xl font-bold text-modal-label-text mb-6">
        {t('set_schedule')}
      </div>

      {sessions.map((session, index) => (
        <div key={session.label} className="mb-5">
          <div className="flex items-center text-modal-label-text text-[15px]/[28px] font-bold mb-1.25">
            {session.label}{' '}
            {session.label !== 'Start' && (
              <span className="ml-1 text-error text-[15px] text-center">*</span>
            )}
          </div>

          <div className="flex flex-wrap items-center gap-2.5 justify-start">
            <div className="shrink-0 max-tablet:w-full">
              <CalendarDropdown
                value={session.start}
                onChange={(v) => {
                  const delta = session.end - session.start;
                  const nextStart = Math.floor(v);
                  const nextEnd = nextStart + delta;
                  setSessions((prev) => {
                    const updated = [...prev];
                    updated[index] = {
                      ...updated[index],
                      start: nextStart,
                      end: nextEnd,
                    };
                    return updated;
                  });
                }}
              />
            </div>

            <div className="shrink-0 max-tablet:w-full max-tablet:mb-[10px]">
              <TimeDropdown
                value={session.start}
                onChange={(v) => {
                  const delta = session.end - session.start;
                  const nextStart = Math.floor(v);
                  const nextEnd = nextStart + delta;
                  setSessions((prev) => {
                    const updated = [...prev];
                    updated[index] = {
                      ...updated[index],
                      start: nextStart,
                      end: nextEnd,
                    };
                    return updated;
                  });
                }}
              />
            </div>

            <span className="px-1 text-neutral-500 max-tablet:hidden">–</span>

            <div className="shrink-0 max-tablet:w-full">
              <CalendarDropdown
                value={session.end}
                onChange={(v) => {
                  if (v < session.start) {
                    showErrorToast(t('end_date_warning'));
                    return;
                  }
                  handleChange(index, 'end', Math.floor(v));
                }}
              />
            </div>

            <div className="shrink-0 max-tablet:w-full max-tablet:mb-[10px]">
              <TimeDropdown
                value={session.end}
                onChange={(v) => {
                  if (v < session.start) {
                    showErrorToast(t('end_date_warning'));
                    return;
                  }
                  handleChange(index, 'end', Math.floor(v));
                }}
              />
            </div>

            <div className="ml-auto max-tablet:ml-0 flex items-center w-fit max-tablet:w-full border border-select-date-border bg-select-date-bg rounded-lg px-5 py-[10.5px] gap-2.5">
              <div className="font-medium text-[15px]/[22.5px] text-neutral-600">
                Pacific Time
              </div>
              <Internet
                className="w-5 h-5 [&>path]:stroke-neutral-600 [&>circle]:stroke-neutral-600"
                width="20"
                height="20"
              />
            </div>
          </div>
        </div>
      ))}

      {/* <div className="mt-6 border-t border-neutral-700 pt-5">
        <div className="flex flex-row w-full items-start gap-2.5">
          <CustomCheckbox
            checked={autoSchedule}
            onChange={() => setAutoSchedule(!autoSchedule)}
            disabled={false}
          />
          <div className="text-[15px]/[24px]">
            <div className="font-medium text-white">Auto Schedule</div>
            <div className="font-normal text-neutral-300">
              Each session is set to start and end automatically based on its
              scheduled date and time.
            </div>
          </div>
        </div>
      </div> */}

      <div className="flex justify-end mt-6">
        <button
          onClick={handleConfirm}
          className="w-fit px-10 py-[14.5px] rounded-[10px] bg-primary hover:bg-hover text-black text-bold text-base hover:text-black cursor-pointer"
        >
          {t('confirm')}
        </button>
      </div>
    </div>
  );
}
