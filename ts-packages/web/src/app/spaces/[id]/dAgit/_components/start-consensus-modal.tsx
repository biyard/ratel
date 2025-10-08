'use client';

import { usePopup } from '@/lib/contexts/popup-service';
import { Button } from '@/components/ui/button';

const openStartConsensusModal = (
  popup: ReturnType<typeof usePopup>,
  artworkTitle: string,
  totalOracles: number,
  handleStart: () => Promise<void>,
) => {
  const handleClose = () => {
    popup.close();
  };
  popup
    .open(
      <StartConsensusModal
        handleStart={handleStart}
        artworkTitle={artworkTitle}
        totalOracles={totalOracles}
        handleClose={handleClose}
      />,
    )
    .withTitle('Start Consensus');
};

export { openStartConsensusModal };

export default function StartConsensusModal({
  handleClose,
  handleStart,
  artworkTitle,
  totalOracles,
}: {
  handleClose: () => void;
  handleStart: () => Promise<void>;
  artworkTitle: string;
  totalOracles: number;
}) {
  return (
    <div className="flex flex-col gap-10 w-[40vw] max-w-200 max-h-[80vh]">
      <div className="flex flex-col gap-4">
        <p className="text-lg text-neutral-500 break-keep">
          You are about to start a consensus for the artwork{'  '}
          <strong className="font-extrabold text-primary">
            ({artworkTitle})
          </strong>
          .
        </p>

        <p>Total Target Oracles: {totalOracles}</p>
      </div>
      <div className="flex flex-row gap-5 w-full [&>*]:flex-1">
        <Button variant="rounded_secondary" onClick={handleClose}>
          Cancel
        </Button>
        <Button variant="rounded_primary" onClick={handleStart}>
          Start Consensus
        </Button>
      </div>
    </div>
  );
}
