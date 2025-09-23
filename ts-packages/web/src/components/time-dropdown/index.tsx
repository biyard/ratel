'use client';

import { useState } from 'react';
import { Clock } from 'lucide-react';
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@radix-ui/react-popover';
interface TimeDropdownProps {
  value: number;
  onChange: (newTimestamp: number) => void;
}

const timeOptions = Array.from({ length: 24 }, (_, i) => {
  const hour = i % 12 || 12;
  const suffix = i < 12 ? 'AM' : 'PM';
  return `${hour.toString().padStart(2, '0')}:00 ${suffix}`;
});

const formatAMPM = (timestamp: number): string => {
  const date = new Date(timestamp);
  const hour = date.getHours();
  const h = hour % 12 || 12;
  const suffix = hour < 12 ? 'AM' : 'PM';
  return `${h.toString().padStart(2, '0')}:00 ${suffix}`;
};

export default function TimeDropdown({ value, onChange }: TimeDropdownProps) {
  const [open, setOpen] = useState(false);
  const selectedTime = value ? formatAMPM(value) : null;

  const handleSelect = (time: string) => {
    const [hourStr, period] = time.split(' ');
    let hour = parseInt(hourStr.split(':')[0], 10);
    if (period === 'AM') {
      if (hour === 12) hour = 0;
    } else {
      if (hour !== 12) hour += 12;
    }

    const old = new Date(value);
    const newDate = new Date(
      old.getFullYear(),
      old.getMonth(),
      old.getDate(),
      hour,
      0,
      0,
      0,
    );

    onChange(newDate.getTime());
    setOpen(false);
  };

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <button className="flex justify-between items-center w-[150px] max-tablet:w-full border border-c-wg-70 rounded-lg px-[20px] py-[10.5px] font-medium text-neutral-600 border-select-date-border bg-select-date-bg text-[15px]/[22.5px] text-left shadow-sm focus:outline-none gap-[10px]">
          {selectedTime || 'Select'}
          <Clock className="w-5 h-5 stroke-neutral-500" />
        </button>
      </PopoverTrigger>

      <PopoverContent
        className="mt-1 w-[150px] rounded-md shadow-lg bg-white max-h-60 overflow-auto border border-gray-200 z-[999]"
        align="start"
        sideOffset={4}
      >
        {timeOptions.map((time) => (
          <div
            key={time}
            onClick={() => handleSelect(time)}
            className={`px-4 py-2 cursor-pointer text-sm text-black hover:bg-gray-100 ${
              time === selectedTime ? 'font-bold' : ''
            }`}
          >
            {time}
          </div>
        ))}
      </PopoverContent>
    </Popover>
  );
}
