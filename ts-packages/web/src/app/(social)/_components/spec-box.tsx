'use client';
import { ChevronRight } from 'lucide-react';

export default function SpecBox({
  left_text,
  action_text,
  onClick,
  'data-pw': dataPw,
}: {
  left_text: string;
  action_text?: string;
  onClick?: () => void;
  'data-pw'?: string;
}) {
  return (
    <div data-pw={dataPw} className="flex items-center justify-between border border-setting-card-border px-4 py-8 rounded-md">
      <p className="text-base font-bold text-text-primary">{left_text}</p>
      <button
        data-pw={dataPw ? `${dataPw}-button` : undefined}
        className="flex items-center gap-2 text-primary cursor-pointer"
        onClick={onClick}
      >
        {action_text}
        <ChevronRight className="w-4 h-4" />
      </button>
    </div>
  );
}
