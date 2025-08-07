'use client';
// https://www.figma.com/design/YaLSz7dzRingD7CipyaC47/Ratel?node-id=4014-113593&t=riEhxEnpWA7Fr3v9-4

import { Button } from '@/components/ui/button';
import { usePopup } from '@/lib/contexts/popup-service';

import React from 'react';

export const openModal = (
  popup: ReturnType<typeof usePopup>,
  makePublic: () => void,
) => {
  popup
    .open(
      <MakePublicModal
        makePublic={makePublic}
        onCancel={() => popup.close()}
      />,
    )
    .withoutBackdropClose();
};
export default function MakePublicModal({
  makePublic,
  onCancel,
}: {
  makePublic: () => void;
  onCancel: () => void;
}) {
  return (
    <div className="max-w-125 flex flex-col mt-6 gap-6">
      <div className="text-center font-bold text-white text-[24px]">
        You’re About to Go Public
      </div>

      <div className="text-center font-medium text-neutral-400 text-base">
        Once made public, this Space will be visible to everyone
        <br />
        and <span className="font-bold">cannot be made private again.</span>
      </div>

      <div className="flex flex-row gap-4 h-12">
        <Button
          variant="outline"
          className="flex-1/3 border-transparent"
          onClick={makePublic}
        >
          Cancel
        </Button>
        <Button
          variant="default"
          className="flex-2/3 bg-primary"
          onClick={onCancel}
        >
          Make Public
        </Button>
      </div>
    </div>
  );
}
