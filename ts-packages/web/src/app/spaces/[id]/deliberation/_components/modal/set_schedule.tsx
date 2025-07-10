'use client';

import React, { useState } from 'react';
import CustomCheckbox from '@/components/checkbox/custom-checkbox';
import { Internet } from '@/components/icons';
import { showErrorToast } from '@/lib/toast';
import TimeDropdown from '@/components/time-dropdown';
import CalendarDropdown from '@/components/calendar-dropdown';

export default function SetSchedulePopup({
  startedAt,
  endedAt,
  onconfirm,
}: {
  startedAt: number;
  endedAt: number;
  onconfirm: (startDate: number, endDate: number) => void;
}) {
  const [autoSchedule, setAutoSchedule] = useState(true);

  // const initialDate = Math.floor(Date.now());

  const [sessions, setSessions] = useState([
    { label: 'Poll', start: startedAt * 1000, end: endedAt * 1000 },
  ]);

  const [calendarStates, setCalendarStates] = useState(
    sessions.map(() => ({ start: false, end: false })),
  );

  const [dropdownStates, setDropdownStates] = useState(
    sessions.map(() => ({ start: false, end: false })),
  );

  const handleChange = (index: number, key: 'start' | 'end', value: number) => {
    const updated = [...sessions];
    updated[index][key] = value;
    setSessions(updated);
  };

  const setCalendarOpenAt = (
    index: number,
    key: 'start' | 'end',
    open: boolean,
  ) => {
    const updated = [...calendarStates];
    updated[index][key] = open;
    setCalendarStates(updated);
  };

  const setDropdownOpenAt = (
    index: number,
    key: 'start' | 'end',
    open: boolean,
  ) => {
    const updated = [...dropdownStates];
    updated[index][key] = open;
    setDropdownStates(updated);
  };

  const handleConfirm = () => {
    for (const s of sessions) {
      if (s.start >= s.end) {
        showErrorToast('Start date must be earlier than end date.');
        return;
      }
    }

    onconfirm(sessions[0].start, sessions[0].end);
  };

  return (
    <div className="w-full max-w-[900px] bg-[#1E1E1E] p-8 rounded-2xl">
      <div className="text-2xl font-bold text-white mb-6">Set Schedule</div>

      {sessions.map((session, index) => (
        <div key={session.label} className="mb-5">
          <div className="flex items-center text-neutral-400 text-[15px]/[28px] font-bold mb-1.25">
            {session.label}{' '}
            {session.label !== 'Start' && (
              <span className="ml-1 text-error text-[15px] text-center">*</span>
            )}
          </div>

          <div className="flex flex-row gap-2.5 items-center">
            <CalendarDropdown
              value={session.start}
              calendarOpen={calendarStates[index].start}
              setCalendarOpen={(v) => {
                setCalendarOpenAt(index, 'start', v);
                setDropdownOpenAt(index, 'start', false);
              }}
              onChange={(v) => {
                handleChange(index, 'start', Math.floor(v));
                setCalendarOpenAt(index, 'start', false);
                setDropdownOpenAt(index, 'start', false);
              }}
            />
            <TimeDropdown
              value={session.start}
              timeDropdownOpen={dropdownStates[index].start}
              setTimeDropdownOpen={(v) => {
                setDropdownOpenAt(index, 'start', v);
                setCalendarOpenAt(index, 'start', false);
              }}
              onChange={(v) => {
                handleChange(index, 'start', Math.floor(v));
                setDropdownOpenAt(index, 'start', false);
                setCalendarOpenAt(index, 'start', false);
              }}
            />

            <div className="w-[15px] h-0.25 bg-neutral-600" />

            <CalendarDropdown
              value={session.end}
              calendarOpen={calendarStates[index].end}
              setCalendarOpen={(v) => {
                setCalendarOpenAt(index, 'end', v);
                setDropdownOpenAt(index, 'end', false);
              }}
              onChange={(v) => {
                handleChange(index, 'end', Math.floor(v));
                setCalendarOpenAt(index, 'end', false);
                setDropdownOpenAt(index, 'end', false);
              }}
            />
            <TimeDropdown
              value={session.end}
              timeDropdownOpen={dropdownStates[index].end}
              setTimeDropdownOpen={(v) => {
                setDropdownOpenAt(index, 'end', v);
                setCalendarOpenAt(index, 'end', false);
              }}
              onChange={(v) => {
                handleChange(index, 'end', Math.floor(v));
                setDropdownOpenAt(index, 'end', false);
                setCalendarOpenAt(index, 'end', false);
              }}
            />

            <div className="flex flex-row items-center w-fit border border-c-wg-70 rounded-lg px-5 py-[10.5px] gap-2.5">
              <div className="font-medium text-[15px]/[22.5px] text-neutral-600">
                Pacific Time
              </div>
              <Internet
                className="w-5 h-5 [&>path]:stroke-neutral-500 [&>circle]:stroke-neutral-500"
                width="20"
                height="20"
              />
            </div>
          </div>
        </div>
      ))}

      <div className="mt-6 border-t border-neutral-700 pt-5">
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
      </div>

      <div className="flex justify-end mt-6">
        <button
          onClick={handleConfirm}
          className="w-fit px-10 py-[14.5px] rounded-[10px] bg-primary hover:bg-hover text-black text-bold text-base hover:text-black cursor-pointer"
        >
          Confirm
        </button>
      </div>
    </div>
  );
}
