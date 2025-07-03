'use client';

import { format } from 'date-fns';
import 'react-day-picker/dist/style.css';
import React, { useEffect, useState } from 'react';
import CalendarDayPicker from './calendar';
import { Calendar } from 'lucide-react';

interface CustomCalendarProps {
  value: number;
  calendarOpen: boolean;
  setCalendarOpen: (open: boolean) => void;
  onChange: (date: number) => void;
}

export default function CustomCalendar({
  value,
  calendarOpen,
  setCalendarOpen,
  onChange,
}: CustomCalendarProps) {
  const [selected, setSelected] = useState<Date>();

  useEffect(() => {
    if (value) setSelected(new Date(value));
  }, [value]);

  return (
    <div className="relative w-fit">
      <button
        className="flex flex-row items-center border border-input px-6 py-[10px] rounded-md font-semibold text-sm text-neutral-300 bg-transparent gap-[20px]"
        onClick={() => {
          setCalendarOpen(!calendarOpen);
        }}
      >
        {selected ? format(selected, 'yyyy/MM/dd') : 'Selected Date'}
        <Calendar className="w-6 h-6 stroke-input" />
      </button>

      {calendarOpen && (
        <div className="absolute mt-2 z-10 bg-white text-black rounded-xl shadow-xl p-4">
          <CalendarDayPicker
            value={value}
            onChange={(date) => {
              if (date) onChange(date.getTime());
              setCalendarOpen(false);
            }}
          />
        </div>
      )}
    </div>
  );
}
