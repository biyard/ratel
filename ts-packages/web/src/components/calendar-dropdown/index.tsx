'use client';

import { format } from 'date-fns';
import 'react-day-picker/dist/style.css';
import React, { useState } from 'react';
import { Calendar } from 'lucide-react';
import CalendarDayPicker from '../calendar-day-picker';
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@radix-ui/react-popover';

interface CalendarDropdownProps {
  value: number;
  onChange: (date: number) => void;
}

export default function CalendarDropdown({
  value,
  onChange,
}: CalendarDropdownProps) {
  const [calendarOpen, setCalendarOpen] = useState<boolean>(false);
  const selectedDate = value ? new Date(value) : null;

  return (
    <Popover open={calendarOpen} onOpenChange={setCalendarOpen}>
      <PopoverTrigger asChild>
        <button className="flex flex-row items-center justify-between w-[150px] border border-c-wg-70 px-[20px] py-[10.5px] rounded-lg font-medium text-[15px]/[22.5px] text-neutral-600 bg-create-space-bg border-[#525252] light:border-create-space-border shadow-sm focus:outline-none gap-[10px]">
          {selectedDate ? format(selectedDate, 'yyyy/MM/dd') : 'Selected Date'}
          <Calendar className="w-5 h-5 stroke-neutral-500" />
        </button>
      </PopoverTrigger>

      <PopoverContent
        className="mt-1 bg-white text-black rounded-xl shadow-xl p-4 w-[350px]"
        align="start"
        sideOffset={4}
      >
        <CalendarDayPicker
          value={value}
          onChange={(date) => {
            if (date) onChange(date.getTime());
            setCalendarOpen(false);
          }}
        />
      </PopoverContent>
    </Popover>
  );
}
