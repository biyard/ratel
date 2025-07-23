'use client';
import React from 'react';

export default function MultiSelectionButton({
  value,
  onChange,
}: {
  value: boolean;
  onChange: (val: boolean) => void;
}) {
  return (
    <label className="flex items-center cursor-pointer gap-2 select-none">
      <span
        className={`font-medium text-[15px]/[24px] ${value ? 'text-blue-500' : 'text-gray-400'}`}
      >
        Multiple selection
      </span>
      <div
        onClick={() => onChange(!value)}
        className={`w-11 h-5 flex items-center bg-blue-500 rounded-full p-1 transition-colors duration-300 ${
          value ? 'bg-blue-500' : 'bg-gray-400'
        }`}
      >
        <div
          className={`bg-white w-3.5 h-3.5 rounded-full shadow-md transform duration-300 ${
            value ? 'translate-x-6' : ''
          }`}
        />
      </div>
    </label>
  );
}
