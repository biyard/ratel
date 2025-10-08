'use client';

import { usePopup } from '@/lib/contexts/popup-service';
import { Button } from '@/components/ui/button';
import { ConsensusVoteType } from '@/lib/api/models/consensus';
import { useState } from 'react';
import { Input } from '@/components/ui/input';


const openVoteModal = (
  popup: ReturnType<typeof usePopup>,
  artworkTitle: string,
  artworkImage: string,

  handleVote: (
    voteType: ConsensusVoteType,
    description: string | null,
  ) => Promise<void>,
) => {
  popup
    .open(
      <VoteModal
        handleVote={handleVote}
        artworkTitle={artworkTitle}
        artworkImage={artworkImage}
      />,
    )
    .withTitle('Vote for Artwork');
};

export { openVoteModal };

export default function VoteModal({
  handleVote,
  artworkTitle,
  artworkImage,
}: {
  handleVote: (
    voteType: ConsensusVoteType,
    description: string | null,
  ) => Promise<void>;
  artworkTitle: string;
  artworkImage: string;
}) {
  const [description, setDescription] = useState<string | null>(null);
  return (
    <div className="flex flex-col gap-10 w-[40vw] max-w-200 max-h-[80vh]">
      <div className="flex flex-col gap-4">
        <p className="text-lg font-extrabold text-primary break-keep">
          {artworkTitle}
        </p>
        <div className="flex flex-col w-full h-full justify-center items-center">
          <div className="relative flex w-full aspect-square">
            <img src={artworkImage} alt={artworkTitle} />
          </div>
        </div>
        <Input
          placeholder="Add a description..."
          value={description || ''}
          onChange={(e) => setDescription(e.target.value)}
        />
      </div>
      <div className="flex flex-row gap-5 w-full [&>*]:flex-1">
        <Button
          variant="rounded_secondary"
          onClick={() => handleVote(ConsensusVoteType.Rejected, description)}
        >
          Reject
        </Button>
        <Button
          variant="rounded_primary"
          onClick={() => handleVote(ConsensusVoteType.Approved, description)}
        >
          Approve
        </Button>
      </div>
    </div>
  );
}
