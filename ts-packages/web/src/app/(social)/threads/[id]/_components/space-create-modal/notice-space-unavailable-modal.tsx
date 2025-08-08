'use client';

import { Remove } from '@/components/icons';
import { usePopup } from '@/lib/contexts/popup-service';
import { useRouter } from 'next/navigation';
import { route } from '@/route';

export default function NoticeSpaceUnavailableModal() {
  const popup = usePopup();
  const router = useRouter();

  const handleCancel = () => {
    popup.close();
  };

  const handleNewFeed = () => {
    // Navigate to Drafts page and close modal
    router.push(route.drafts());
    popup.close();
  };

  return (
    <div className="w-[420px] flex flex-col relative">
      {/* Close button - top right */}
      <button
        onClick={handleCancel}
        className="absolute top-0 right-0 p-2 text-neutral-400 hover:text-white transition-colors"
      >
        <Remove className="w-6 h-6" />
      </button>

      {/* Header */}
      <div className="text-center font-bold text-white text-[24px] mb-6 mt-2">
        Notice Space Unavailable
      </div>

      {/* Body */}
      <div className="text-center font-medium text-neutral-400 text-base mb-8">
        This feed is already public and cannot be converted into a Notice Space.
        <br />
        <br />
        Only private or draft feeds can be used.
        <br />
        Would you like to create a new private feed for this Notice Space?
      </div>

      {/* Buttons */}
      <div className="flex flex-row justify-center gap-4">
        {/* Left button - transparent background like space selection form */}
        <button
          onClick={handleCancel}
          className="flex-1 py-[14.5px] bg-transparent font-bold text-white text-base rounded-[10px] hover:bg-neutral-800 transition-colors"
        >
          Cancel
        </button>

        {/* Right button - primary background */}
        <button
          onClick={handleNewFeed}
          className="flex-1 py-[14.5px] bg-primary font-bold text-black text-base rounded-[10px] hover:bg-primary/90 transition-colors"
        >
          New Feed
        </button>
      </div>
    </div>
  );
}
