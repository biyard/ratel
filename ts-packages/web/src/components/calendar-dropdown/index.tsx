'use client';

import { format } from 'date-fns';
import 'react-day-picker/dist/style.css';
import React, { useState } from 'react';
import { Calendar } from 'lucide-react';
import CalendarDayPicker from '../calendar-day-picker';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from '@radix-ui/react-dropdown-menu';

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
    <DropdownMenu open={calendarOpen} onOpenChange={setCalendarOpen}>
      <DropdownMenuTrigger asChild>
        <button className="flex flex-row items-center justify-between w-[150px] border border-c-wg-70 px-[20px] py-[10.5px] rounded-lg font-medium text-[15px]/[22.5px] text-neutral-600 bg-transparent shadow-sm focus:outline-none gap-[10px]">
          {selectedDate ? format(selectedDate, 'yyyy/MM/dd') : 'Selected Date'}
          <Calendar className="w-5 h-5 stroke-neutral-500" />
        </button>
      </DropdownMenuTrigger>

      <DropdownMenuContent
        className="mt-1 bg-white text-black rounded-xl shadow-xl p-4"
        align="start"
        sideOffset={4}
      >
        <div className="w-full">
          <CalendarDayPicker
            value={value}
            onChange={(date) => {
              if (date) onChange(date.getTime());
              setCalendarOpen(false);
            }}
          />
        </div>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
