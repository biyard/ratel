'use client';

import { Clock } from 'lucide-react';
import React, { useEffect, useState } from 'react';

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
  const [selectedTime, setSelectedTime] = useState(formatAMPM(value));
  const [open, setOpen] = useState(false);

  useEffect(() => {
    setSelectedTime(formatAMPM(value));
  }, [value]);

  const handleSelect = (time: string) => {
    setSelectedTime(time);
    setOpen(false);

    const [hourStr, period] = time.split(' ');
    let hour = parseInt(hourStr.split(':')[0], 10);

    if (period === 'AM') {
      if (hour === 12) hour = 0;
    } else if (period === 'PM') {
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
  };

  return (
    <div className="relative w-fit">
      <button
        className="flex justify-between items-center w-full border border-input rounded-md px-6 py-[10px] font-medium text-neutral-300 text-sm text-left shadow-sm focus:outline-none gap-[20px]"
        onClick={() => setOpen(!open)}
      >
        {selectedTime}
        <Clock className="w-6 h-6 stroke-input" />
      </button>

      {open && (
        <div className="absolute z-10 mt-1 w-full rounded-md shadow-lg bg-white max-h-60 overflow-auto border border-gray-200 text-black">
          {timeOptions.map((time) => (
            <div
              key={time}
              className={`px-4 py-2 cursor-pointer text-sm hover:bg-gray-100 ${
                time === selectedTime ? 'font-bold' : ''
              }`}
              onClick={() => handleSelect(time)}
            >
              {time}
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
