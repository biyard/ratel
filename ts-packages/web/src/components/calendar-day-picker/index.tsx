'use client';
import { format } from 'date-fns';
import React, { useEffect, useState } from 'react';
import { DayPicker } from 'react-day-picker';

export default function CalendarDayPicker({
  value,
  onChange,
}: {
  value: number;
  onChange: (date: Date) => void;
}) {
  const [selected, setSelected] = useState<Date>();

  useEffect(() => {
    if (value) setSelected(new Date(value));
  }, [value]);

  return (
    <>
      <DayPicker
        mode="single"
        selected={selected}
        onSelect={(date) => {
          setSelected(date);
          if (date) onChange(date);
        }}
        modifiersClassNames={{
          selected: 'bg-blue-600 text-white rounded-full',
          today: 'text-blue-600 font-bold',
        }}
        classNames={{
          months: 'flex flex-col',
          month: 'space-y-4',
          caption: 'flex flex-row w-full justify-start items-center mb-2 px-1',
          caption_label:
            'flex flex-row w-full justify-between items-center text-sm font-semibold text-neutral-700',
          nav_button: 'bg-neutral-500 text-white p-1 hover:opacity-80 rounded',
          table: 'w-full table-fixed border-collapse',
          head_row: 'font-bold',
          head_cell: 'text-xs text-center text-neutral-500 font-bold pb-1',
          row: '',
          cell: 'text-center text-sm text-neutral-800 w-10 h-10 p-0 rounded-full hover:bg-blue-100 cursor-pointer',
        }}
        formatters={{
          formatCaption: (date) => format(date, 'MMMM yyyy'),
        }}
      />
      <style jsx global>{`
        .rdp-nav_button svg path {
          stroke: white !important;
          fill: white !important;
        }
      `}</style>
    </>
  );
}
