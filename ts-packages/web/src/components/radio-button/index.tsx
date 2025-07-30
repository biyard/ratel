'use client';

import React from 'react';

export default function RadioButton({
  onClick,
  selected,
}: {
  onClick: () => void;
  selected: boolean;
}) {
  return (
    <div className="flex items-center">
      <button
        onClick={onClick}
        className={`w-6 h-6 rounded-full flex items-center justify-center transition-colors ${
          selected
            ? 'bg-[#fcb300] hover:bg-[#fcb300]/90'
            : 'border-2 border-[#6b6b6b] hover:border-white'
        }`}
      >
        {selected && (
          <svg
            className="w-3 h-3 text-black"
            fill="currentColor"
            viewBox="0 0 20 20"
          >
            <path
              fillRule="evenodd"
              d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z"
              clipRule="evenodd"
            />
          </svg>
        )}
      </button>
    </div>
  );
}
