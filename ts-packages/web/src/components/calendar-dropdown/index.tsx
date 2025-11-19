'use client';

import { format } from 'date-fns';
import 'react-day-picker/dist/style.css';
import { useState } from 'react';
import { Calendar } from 'lucide-react';
import CalendarDayPicker from '../calendar-day-picker';
import * as Popover from '@radix-ui/react-popover';

interface CalendarDropdownProps
  extends Omit<React.HTMLAttributes<HTMLButtonElement>, 'onChange'> {
  value: number;
  onChange: (date: number) => void;
  canEdit?: boolean;
}

export default function CalendarDropdown({
  value,
  onChange,
  canEdit,
  ...restProps
}: CalendarDropdownProps) {
  const [calendarOpen, setCalendarOpen] = useState<boolean>(false);
  const selectedDate = value ? new Date(value) : null;

  return (
    <Popover.Root
      modal={false}
      open={canEdit && calendarOpen}
      onOpenChange={setCalendarOpen}
    >
      <Popover.Trigger asChild>
        <button
          data-testid="calendar-dropdown-trigger"
          className="flex flex-row justify-between items-center font-medium rounded-lg border shadow-sm focus:outline-none w-[150px] max-tablet:w-full px-5 py-[10.5px] text-[15px]/[22.5px] text-neutral-600 bg-select-date-bg border-select-date-border gap-2.5"
          {...restProps}
        >
          {selectedDate ? format(selectedDate, 'yyyy/MM/dd') : 'Selected Date'}
          <Calendar className="w-5 h-5 stroke-neutral-500" />
        </button>
      </Popover.Trigger>

      <Popover.Portal>
        <Popover.Content
          className="p-4 mt-1 text-black bg-white rounded-xl shadow-xl w-[350px] z-[9999]"
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
        </Popover.Content>
      </Popover.Portal>
    </Popover.Root>
  );
}
