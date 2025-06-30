'use client';
import { useRef, useEffect, useState } from 'react';
import { ChevronDownIcon, ChevronUpIcon } from '@heroicons/react/20/solid';
import clsx from 'clsx';
import { Group } from '@/lib/api/models/user';

export interface SelectBoxOption {
  name: string;
  disabled?: boolean;
}

interface SelectBoxProps {
  groups: Group[];
  groupIndex: number;
  setGroupIndex: (index: number) => void;
  selectedGroup: Group;
  setSelectedGroup: (group: Group) => void;
}

export default function SelectBox({
  groups,
  groupIndex,
  setGroupIndex,
  selectedGroup,
  setSelectedGroup,
}: SelectBoxProps) {
  const boxRef = useRef<HTMLDivElement>(null);
  const [isOpen, setIsOpen] = useState(false);

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (boxRef.current && !boxRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    };
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  return (
    <div className="relative w-full max-w-[300px] rounded-lg" ref={boxRef}>
      <button
        className={clsx(
          'w-full flex items-center justify-between px-5 py-3 rounded-md text-sm outline-none transition relative',
          isOpen ? 'border border-yellow-500' : 'border border-[#464646]',
          'bg-transparent text-white font-medium',
        )}
        onClick={() => setIsOpen((prev) => !prev)}
        type="button"
      >
        {selectedGroup.name}
        {isOpen ? (
          <ChevronUpIcon className="w-[15px] h-[15px] text-white absolute right-3 top-1/2 -translate-y-1/2" />
        ) : (
          <ChevronDownIcon className="w-[15px] h-[15px] text-white absolute right-3 top-1/2 -translate-y-1/2" />
        )}
      </button>

      {isOpen && (
        <ul className="absolute left-0 top-full mt-2 w-full bg-[#191919] border border-yellow-500 rounded-md shadow-lg text-sm py-1 z-[9999] overflow-visible h-[100px] overflow-y-scroll">
          {groups.map((option, index) => (
            <li
              key={option.id}
              onClick={() => {
                setSelectedGroup(option);
                setGroupIndex(index);
                setIsOpen(false);
              }}
              className={clsx(
                'cursor-pointer px-4 py-3 select-none flex justify-between items-center',
                index === groupIndex
                  ? 'text-white font-semibold text-[15px]/[16px]'
                  : 'text-[#777677] font-semibold text-[15px]/[16px]',
              )}
            >
              <span>{option.name}</span>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
