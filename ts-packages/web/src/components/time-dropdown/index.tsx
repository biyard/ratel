'use client';

import { useEffect, useRef, useState } from 'react';
import { Clock } from 'lucide-react';
import * as Popover from '@radix-ui/react-popover';

interface TimeDropdownProps
  extends Omit<React.HTMLAttributes<HTMLButtonElement>, 'onChange'> {
  value: number;
  onChange: (newTimestamp: number) => void;
  canEdit?: boolean;
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

export default function TimeDropdown({
  value,
  onChange,
  canEdit,
  ...restProps
}: TimeDropdownProps) {
  const [open, setOpen] = useState(false);
  const selectedTime = value ? formatAMPM(value) : null;

  const triggerRef = useRef<HTMLButtonElement | null>(null);
  const [contentWidth, setContentWidth] = useState<number>(150);
  useEffect(() => {
    const el = triggerRef.current;
    if (!el) return;
    const setW = () => setContentWidth(el.offsetWidth);
    setW();
    const ro = new ResizeObserver(setW);
    ro.observe(el);
    window.addEventListener('resize', setW);
    return () => {
      ro.disconnect();
      window.removeEventListener('resize', setW);
    };
  }, []);

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
    <Popover.Root modal={false} open={canEdit && open} onOpenChange={setOpen}>
      <Popover.Trigger asChild>
        <button
          ref={triggerRef}
          className="flex justify-between items-center font-medium text-left rounded-lg border shadow-sm focus:outline-none w-[150px] max-tablet:w-full border-c-wg-70 px-[20px] py-[10.5px] text-neutral-600 border-select-date-border bg-select-date-bg text-[15px]/[22.5px] gap-[10px]"
          {...restProps}
        >
          {selectedTime || 'Select'}
          <Clock className="w-5 h-5 stroke-neutral-500" />
        </button>
      </Popover.Trigger>

      <Popover.Portal>
        <Popover.Content
          align="start"
          sideOffset={4}
          style={{ width: contentWidth }}
          className="overflow-auto mt-1 max-h-60 bg-white rounded-md border border-gray-200 shadow-lg z-[999]"
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
        </Popover.Content>
      </Popover.Portal>
    </Popover.Root>
  );
}
