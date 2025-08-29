'use client';

import React, { useState } from 'react';
import { Check, X } from 'lucide-react';

export interface ThemeModalProps {
  onCancel: () => void;
  onSave: (theme: 'dark' | 'light' | 'system') => void;
  initialTheme?: 'dark' | 'light' | 'system';
}

export default function ThemeModal({
  onCancel,
  onSave,
  initialTheme = 'dark',
}: ThemeModalProps) {
  const [selectedTheme, setSelectedTheme] = useState<'dark' | 'light' | 'system'>(initialTheme);

  const options: { label: string; value: 'dark' | 'light' | 'system' }[] = [
    { label: 'Dark Theme', value: 'dark' },
    { label: 'Light Theme', value: 'light' },
    { label: 'System Default', value: 'system' },
  ];

  return (
    <div className="w-[420px] ">
      {/* Header */}
      <div className="flex items-center justify-between px-5 py-4  border-neutral-800">
        <p className="text-white font-bold text-lg">Theme</p>
       
      </div>

      {/* Options */}
      <div className="flex flex-col gap-2">
        {options.map((opt, idx) => (
          <button
            key={opt.value}
            onClick={() => setSelectedTheme(opt.value)}
            className={`flex items-center justify-between px-5 py-4 text-left transition-colors rounded-[10px] bg-neutral-800
              ${selectedTheme === opt.value ? 'border border-neutral-400' : 'border border-0 '}
             `}
          >
            <span className="text-white">{opt.label}</span>
            {selectedTheme === opt.value && (
              <Check className="h-5 w-5 text-amber-400" />
            )}
          </button>
        ))}
      </div>

      {/* Footer */}
      <div className="flex flex-row justify-end gap-4 mt-4">
        <button
          onClick={onCancel}
          className="px-10 py-[14.5px] bg-transparent font-bold text-base text-neutral-400 hover:text-white transition-colors"
        >
          Cancel
        </button>
        <button
          onClick={() => onSave(selectedTheme)}
          className="w-full py-[14.5px] font-bold text-base text-bg rounded-[10px] bg-primary"
        >
          Save
        </button>
      </div>
    </div>
  );
}
