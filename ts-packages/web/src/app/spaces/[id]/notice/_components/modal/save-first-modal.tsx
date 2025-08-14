'use client';

import React from 'react';

export interface SaveFirstModalProps {
  onJustPublish: () => void;
  onSaveAndPublish: () => void;
}

export default function SaveFirstModal({
  onJustPublish,
  onSaveAndPublish,
}: SaveFirstModalProps) {
  return (
    <div className="w-[500px] flex flex-col">
      {/* Header */}
      <div className="text-center font-bold text-white text-[24px] mb-6 mt-2">
        Save first, make public?
      </div>

      {/* Body */}
      <div className="text-center font-medium text-neutral-400 text-base mb-8">
        Looks like you haven't saved yet.
        <br />
        Want to save your changes before going public,
        <br />
        or skip it and publish anyway?
        <br />
        <br />
        Once made public, this Space will be visible to everyone and cannot be
        made private again.
      </div>

      {/* Buttons */}
      <div className="flex flex-row justify-center gap-4">
        {/* Left button - transparent background like space selection form */}
        <button
          onClick={onJustPublish}
          className="flex-1 py-[14.5px] bg-transparent font-bold text-white text-base rounded-[10px] hover:bg-neutral-800 transition-colors"
        >
          Just Publish
        </button>

        {/* Right button - primary background */}
        <button
          onClick={onSaveAndPublish}
          className="flex-1 py-[14.5px] bg-primary font-bold text-black text-base rounded-[10px] hover:bg-primary/90 transition-colors"
        >
          Save & Publish
        </button>
      </div>
    </div>
  );
}
