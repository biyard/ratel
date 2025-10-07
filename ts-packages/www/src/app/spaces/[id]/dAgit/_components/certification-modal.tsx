'use client';

import { usePopup } from '@/lib/contexts/popup-service';
import { ConsensusVoteType } from '@/lib/api/models/consensus';
import { getTimeWithFormat } from '@/lib/time-utils';
import { ArtworkCertificate } from '@/lib/api/models/artwork';

const openCertificationModal = (
  popup: ReturnType<typeof usePopup>,
  certificate: ArtworkCertificate,
) => {
  popup
    .open(<CertificationModal items={certificate} />)
    .withTitle('Certificate');
};

export { openCertificationModal };

export default function CertificationModal({
  items,
}: {
  items: ArtworkCertificate;
}) {
  return (
    <div className="flex flex-col gap-10 w-[25vw] max-w-200 max-h-[80vh]">
      <div className="flex flex-col gap-4">
        <h3 className="text-lg font-semibold">
          Certificate Time: {getTimeWithFormat(items.certified_at)}
        </h3>
        <p>Total Oracles: {items.total_oracles}</p>
        <p>Total Votes: {items.total_votes}</p>
        <p>Approved Votes: {items.approved_votes}</p>
        <p>Rejected Votes: {items.rejected_votes}</p>
        <p className="text-lg font-bold">Oracle Information</p>
        <div className="flex flex-col gap-2 h-full overflow-y-scroll border border-neutral-700 p-4 rounded-lg">
          {items.voters.map((voter, voterIndex) => (
            <div key={voterIndex} className="flex flex-row gap-2">
              <div>
                {voter.nickname}:{' '}
                <span className="font-bold">
                  {voteTypeToText(voter.vote_type)}
                  {voter.description && <span>({voter.description})</span>}
                </span>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

const voteTypeToText = (voteType: ConsensusVoteType | null): string => {
  switch (voteType) {
    case ConsensusVoteType.Approved:
      return 'Approved';
    case ConsensusVoteType.Rejected:
      return 'Rejected';
    default:
      return 'Unknown';
  }
};
