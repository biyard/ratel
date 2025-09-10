'use client';

import React, { useState } from 'react';
import CheckCircle from '@/assets/icons/check-circle.svg';

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
  const [selectedTheme, setSelectedTheme] = useState<
    'dark' | 'light' | 'system'
  >(initialTheme);

  const options: { label: string; value: 'dark' | 'light' | 'system' }[] = [
    { label: 'Dark Theme', value: 'dark' },
    { label: 'Light Theme', value: 'light' },
    { label: 'System Default', value: 'system' },
  ];

  return (
    <div className="w-[420px]">
      {/* Options */}
      <div className="flex flex-col gap-2">
        {options.map((opt) => (
          <button
            key={opt.value}
            onClick={() => setSelectedTheme(opt.value)}
            className={`flex items-center justify-between px-5 py-4 text-left transition-colors rounded-[10px] light:bg-white
              ${selectedTheme === opt.value ? 'border border-neutral-400 light:border-primary light:bg-primary/10' : 'border-modal-card-border bg-modal-card-bg'}
             `}
          >
            <span className="text-text-primary">{opt.label}</span>
            {selectedTheme === opt.value && (
              <CheckCircle className="h-5 w-5 [&>circle]:hidden [&>path]:stroke-primary" />
            )}
          </button>
        ))}
      </div>

      {/* Footer */}
      <div className="flex flex-row justify-end gap-4 mt-4">
        <button
          onClick={onCancel}
          className="px-10 py-[14.5px] bg-cancel-button-bg font-bold text-base text-cancel-button-text hover:text-cancel-button-text/80 transition-colors"
        >
          Cancel
        </button>
        <button
          onClick={() => onSave(selectedTheme)}
          className="w-full py-[14.5px] font-bold text-base text-submit-button-text rounded-[10px] bg-submit-button-bg hover:bg-submit-button-bg/80"
        >
          Save
        </button>
      </div>
    </div>
  );
}
