'use client';
import React from 'react';

export default function CheckPopup({
  onContinue,
  onClose,
}: {
  onContinue: () => void;
  onClose: () => void;
}) {
  return (
    <div className="w-full max-w-[450px] px-[20px]">
      <div className="flex flex-col gap-[40px]">
        <div className="font-semibold text-base text-white">
          For the integrity and security of the vote, all submissions are
          anonymous and final. Votes cannot be changed once cast, and we
          appreciate your understanding in ensuring reliable results.
        </div>

        <div className="flex flex-row w-full justify-end items-center gap-[10px]">
          <div
            className="cursor-pointer flex flex-row w-fit h-fit px-[14px] py-[8px] rounded-lg bg-primary hover:opacity-50 font-semibold text-sm text-[#000203]"
            onClick={() => onContinue()}
          >
            Confirm
          </div>
          <div
            className="cursor-pointer flex flex-row w-fit h-fit px-[14px] py-[8px] rounded-lg bg-neutral-500 hover:opacity-50 font-semibold text-sm text-white"
            onClick={() => onClose()}
          >
            Cancel
          </div>
        </div>
      </div>
    </div>
  );
}
