'use client';

// https://www.figma.com/design/YaLSz7dzRingD7CipyaC47/Ratel?node-id=4014-115291&t=riEhxEnpWA7Fr3v9-4

import { Button } from '@/components/ui/button';
import { usePopup } from '@/lib/contexts/popup-service';

import React from 'react';

export const openModal = (
  popup: ReturnType<typeof usePopup>,
  makePublic: () => Promise<void>,
  saveSpace: () => Promise<void>,
) =>
  popup
    .open(
      <MakePublicWithSavingModal
        makePublic={() => {
          makePublic();
        }}
        makePublicWithSave={() => {
          makePublic().then(() => saveSpace());
        }}
      />,
    )
    .withoutBackdropClose();

export default function MakePublicWithSavingModal({
  makePublic,
  makePublicWithSave,
}: {
  makePublic: () => void;
  makePublicWithSave: () => void;
}) {
  return (
    <div className="max-w-125 flex flex-col mt-6 gap-6">
      <div className="text-center font-bold text-white text-[24px]">
        Save first, make public?
      </div>
      <div className="text-center font-medium text-neutral-400 text-[16px]">
        Looks like you haven’t saved yet.
        <br />
        Want to save your changes before going public.
        <br />
        or skip it and publish anyway?
        <br />
        <br />
        Once made public, this Space will be visible to everyone and
        <span className="font-bold">cannot be made private again.</span>
      </div>

      <div className="flex flex-row gap-4 h-12">
        <Button
          variant="outline"
          onClick={makePublic}
          className="flex-1/3 border-transparent"
          // className="px-10 py-[14.5px] bg-transparent font-bold text-base text-neutral-400"
        >
          Just Publish
        </Button>
        <Button
          variant="default"
          onClick={makePublicWithSave}
          className="flex-2/3 bg-primary"
          // className="w-full py-[14.5px] bg-primary font-bold text-black text-base rounded-[10px]"
        >
          Save & Publish
        </Button>
      </div>
    </div>
  );
}
