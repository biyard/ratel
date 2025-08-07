'use client';

import React from 'react';

export default function GoPublicPopup({
  onpublic,
  onclose,
}: {
  onpublic: () => void;
  onclose: () => void;
}) {
  return (
    <div className="w-[500px] flex flex-col mt-6">
      <div className="text-center font-bold text-white text-[24px] mb-6">
        You’re About to Go Public
      </div>

      <div className="text-center font-medium text-neutral-400 text-base">
        Once made public, this Space will be visible to everyone
        <br />
        and <span className="font-bold">cannot be made private again.</span>
      </div>

      <div className="flex flex-row justify-end gap-4 mt-8.75">
        <button
          onClick={onclose}
          className="px-10 py-[14.5px] bg-transparent font-bold text-base text-neutral-400"
        >
          Cancel
        </button>
        <button
          onClick={onpublic}
          className="w-full py-[14.5px] bg-primary font-bold text-black text-base rounded-[10px]"
        >
          Make Public
        </button>
      </div>
    </div>
  );
}
