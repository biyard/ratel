'use client';

import React from 'react';
import { Remove } from '@/components/icons';

export interface GoPublicModalProps {
  onCancel: () => void;
  onGoPublic: () => void;
}

export default function GoPublicModal({
  onCancel,
  onGoPublic,
}: GoPublicModalProps) {
  return (
    <div className="w-[500px] flex flex-col relative">
      {/* Close button - top right */}
      <button
        onClick={onCancel}
        className="absolute top-0 right-0 p-2 text-neutral-400 hover:text-white transition-colors"
      >
        <Remove className="w-6 h-6" />
      </button>

      {/* Header */}
      <div className="text-center font-bold text-white text-[24px] mb-6 mt-2">
        You're About to Go Public
      </div>

      {/* Body */}
      <div className="text-center font-medium text-neutral-400 text-base mb-8">
        Once made public, this Space will be visible to everyone and cannot be
        made private again.
      </div>

      {/* Buttons */}
      <div className="flex flex-row justify-center gap-4">
        {/* Left button - transparent background like space selection form */}
        <button
          onClick={onCancel}
          className="flex-1 py-[14.5px] bg-transparent font-bold text-white text-base rounded-[10px] hover:bg-neutral-800 transition-colors"
        >
          Cancel
        </button>

        {/* Right button - primary background */}
        <button
          onClick={onGoPublic}
          className="flex-1 py-[14.5px] bg-primary font-bold text-black text-base rounded-[10px] hover:bg-primary/90 transition-colors"
        >
          Go Public
        </button>
      </div>
    </div>
  );
}
