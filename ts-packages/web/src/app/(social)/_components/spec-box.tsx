'use client';
import { ChevronRight } from 'lucide-react';
import React from 'react';

export default function SpecBox({
  left_text,
  action_text,
  onClick,
}: {
  left_text: string;
  action_text?: string;
  onClick?: () => void;
}) {
  return (
    <div className="flex items-center justify-between border border-setting-card-border px-4 py-8 rounded-md">
      <p className="text-base font-bold text-text-primary">{left_text}</p>
      <button
        className="flex items-center gap-2 text-primary cursor-pointer"
        onClick={onClick}
      >
        {action_text}
        <ChevronRight className="w-4 h-4" />
      </button>
    </div>
  );
}
