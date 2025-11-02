'use client';

import { useState } from 'react';
import { Internet } from '@/components/icons';
import * as Popover from '@radix-ui/react-popover';
import { ChevronDown, Search } from 'lucide-react';

interface TimezoneDropdownProps {
  value: string;
  onChange: (timezone: string) => void;
  canEdit?: boolean;
}

// Get list of all available timezones
const getTimezones = (): string[] => {
  return Intl.supportedValuesOf('timeZone');
};

export default function TimezoneDropdown({
  value,
  onChange,
  canEdit = true,
}: TimezoneDropdownProps) {
  const [dropdownOpen, setDropdownOpen] = useState<boolean>(false);
  const [searchQuery, setSearchQuery] = useState<string>('');
  const timezones = getTimezones();

  const filteredTimezones = timezones.filter((tz) =>
    tz.toLowerCase().includes(searchQuery.toLowerCase()),
  );

  const handleSelect = (timezone: string) => {
    onChange(timezone);
    setDropdownOpen(false);
    setSearchQuery('');
  };

  return (
    <Popover.Root
      open={canEdit && dropdownOpen}
      onOpenChange={canEdit ? setDropdownOpen : undefined}
      modal={false}
    >
      <Popover.Trigger asChild>
        <button
          className="flex flex-row gap-2.5 items-center px-5 mt-2 w-full rounded-lg border sm:mt-0 border-select-date-border bg-select-date-bg py-[10.5px] sm:w-fit hover:border-neutral-500 transition-colors disabled:cursor-not-allowed disabled:opacity-50"
          disabled={!canEdit}
        >
          <div className="font-medium text-[15px]/[22.5px] text-neutral-600 whitespace-nowrap">
            {value}
          </div>
          <Internet
            className="w-5 h-5 [&>path]:stroke-neutral-600 [&>circle]:stroke-neutral-600"
            width="20"
            height="20"
          />
          {canEdit && (
            <ChevronDown className="w-4 h-4 stroke-neutral-600 ml-auto" />
          )}
        </button>
      </Popover.Trigger>

      <Popover.Portal>
        <Popover.Content
          className="p-2 mt-1 bg-white rounded-xl shadow-xl w-[350px] z-[9999] max-h-[400px] flex flex-col"
          align="start"
          sideOffset={4}
        >
          <div className="p-2 border-b border-gray-200">
            <div className="flex items-center gap-2 px-3 py-2 bg-gray-50 rounded-lg">
              <Search className="w-4 h-4 text-gray-400" />
              <input
                type="text"
                placeholder="Search timezone..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
                className="flex-1 bg-transparent border-none outline-none text-sm text-gray-700 placeholder-gray-400"
                autoFocus
              />
            </div>
          </div>
          <div className="overflow-y-auto max-h-[300px]">
            {filteredTimezones.length === 0 ? (
              <div className="p-4 text-center text-sm text-gray-500">
                No timezones found
              </div>
            ) : (
              filteredTimezones.map((tz) => (
                <button
                  key={tz}
                  onClick={() => handleSelect(tz)}
                  className={`w-full text-left px-4 py-2 text-sm hover:bg-gray-100 transition-colors ${
                    tz === value ? 'bg-blue-50 text-blue-700 font-medium' : 'text-gray-700'
                  }`}
                >
                  {tz}
                </button>
              ))
            )}
          </div>
        </Popover.Content>
      </Popover.Portal>
    </Popover.Root>
  );
}
