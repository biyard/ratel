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
        className="flex flex-row items-center border border-c-wg-70 px-[20px] py-[10.5px] rounded-lg font-medium text-[15px]/[22.5px] text-neutral-600 bg-transparent gap-[10px] z-100"
        onClick={() => {
          setCalendarOpen(!calendarOpen);
        }}
      >
        {selected ? format(selected, 'yyyy/MM/dd') : 'Selected Date'}
        <Calendar className="w-5 h-5 stroke-neutral-500" />
      </button>

      {calendarOpen && (
        <div className="absolute mt-2 z-100 bg-white text-black rounded-xl shadow-xl p-4">
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
