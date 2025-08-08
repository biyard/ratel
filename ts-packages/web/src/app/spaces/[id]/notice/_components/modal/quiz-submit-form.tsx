'use client';

import React from 'react';
import { Remove } from '@/components/icons';

export interface QuizSubmitFormProps {
  onSubmit: () => void;
  onClose: () => void;
}

export default function QuizSubmitForm({
  onSubmit,
  onClose,
}: QuizSubmitFormProps) {
  return (
    <div className="w-[500px] flex flex-col mt-2 relative">
      {/* Close button - top right */}
      <button
        onClick={onClose}
        className="absolute top-0 right-0 p-2 text-neutral-400 hover:text-white transition-colors"
        aria-label="Close"
      >
        <Remove className="w-5 h-5 stroke-[currentColor]" />
      </button>

      <div className="text-center font-bold text-white text-[24px] mb-6">
        Just heads up!
      </div>

      {/* Warning Content */}
      <div className="text-center font-medium text-neutral-400 text-base">
        Wrong answers will reduce your reward by{' '}
        <span className="text-red-500 font-bold">50%</span>
        <br />
        You can test again, but remember — the{' '}
        <span className="font-bold">
          reward will be halved each time you do.
        </span>
        <br />
        <br />
        <span className="text-white text-lg font-medium">
          Submit your answer anyway?
        </span>
      </div>

      {/* Action Buttons */}
      <div className="flex flex-row justify-end gap-4 mt-8.75">
        <button
          onClick={onClose}
          className="px-10 py-[14.5px] bg-transparent font-bold text-base text-neutral-400"
        >
          Cancel
        </button>
        <button
          onClick={onSubmit}
          className="w-full py-[14.5px] bg-primary font-bold text-black text-base rounded-[10px]"
        >
          Submit
        </button>
      </div>
    </div>
  );
}
