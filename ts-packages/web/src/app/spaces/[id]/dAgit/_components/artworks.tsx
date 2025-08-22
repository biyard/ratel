/* eslint-disable @typescript-eslint/no-unused-vars */
'use client';
import { Button } from '@/components/ui/button';
import { useArtworkDetailById } from '@/hooks/use-artwork-detail';
import Artwork from '@/lib/api/models/artwork';
import Image from 'next/image';
import { useState } from 'react';
import { openCreateArtworkModal } from './create-artwork-modal';
import { usePopup } from '@/lib/contexts/popup-service';
import { useEditCoordinatorStore } from '../../space-store';
import { useDagitStore } from '../dagit-store';
import Certified from '@/assets/icons/certified.svg';
import { openCertificationModal } from './certification-modal';
import { useArtworkCertificateById } from '@/hooks/use-artwork-certificate';
import useDagitBySpaceId, { useDagitByIdMutation } from '@/hooks/use-dagit';
import { openStartConsensusModal } from './start-consensus-modal';
import { ConsensusVoteType } from '@/lib/api/models/consensus';
import { openVoteModal } from './vote-modal';
import { useUserInfo } from '@/app/(social)/_hooks/user';

function ArtworkViewer({
  artwork,
  isTemporary = false,
  totalOracles = 0,
  isOracle = false,
  isOwner = false,
  handleStartConsensus = async (_artworkId: number) => {},
  handleVote = async (
    _artworkId: number,
    _description: string | null,
    _voteType: ConsensusVoteType,
  ) => {},
}: {
  artwork: Artwork;
  isTemporary?: boolean;
  totalOracles?: number;
  isOracle?: boolean;
  isOwner?: boolean;

  handleStartConsensus?: (artworkId: number) => Promise<void>;
  handleVote?: (
    artworkId: number,
    description: string | null,
    voteType: ConsensusVoteType,
  ) => Promise<void>;
}) {
  console.log('ArtworkViewer', artwork.file.url);
  const { data: original } = useArtworkDetailById(
    artwork.id,
    !isTemporary && isOwner,
  );
  const { data: certificate } = useArtworkCertificateById(
    artwork.id,
    !isTemporary && artwork.is_certified,
  );
  const popup = usePopup();
  const [showOriginal, setShowOriginal] = useState(false);

  return (
    <div className="flex flex-col gap-4 border border-primary rounded p-4">
      <div className="flex justify-between items-center">
        <h3 className="text-lg font-semibold flex flex-row gap-2 items-center">
          {artwork.title}
          {artwork.is_certified && <Certified />}
          {isTemporary && ' (Temporary)'}
        </h3>
        {!isTemporary && original && (
          <Button
            size="sm"
            variant="rounded_primary"
            disabled={!original || !original.image}
            onClick={() => setShowOriginal((prev) => !prev)}
          >
            {showOriginal ? 'Hide Original' : 'Show Original'}
          </Button>
        )}
      </div>
      <div className="flex flex-col w-full h-full justify-center items-center">
        <div className="relative w-full max-h-128 object-cover">
          {showOriginal && original?.image && (
            <Image
              src={original.image}
              alt={artwork.title}
              width={500}
              height={500}
              className="object-cover"
            />
          )}
          {!showOriginal && !!artwork.file.url && (
            <Image
              src={artwork.file.url}
              alt={artwork.title}
              width={500}
              height={500}
              className="object-cover"
            />
          )}
          {!artwork.file.url && !showOriginal && (
            <span className="w-full flex text-center">
              No Thumbnail Available
            </span>
          )}
        </div>
      </div>
      <div className="self-end">
        {!isTemporary && !artwork.is_certified && !artwork.has_consensus && (
          <Button
            variant="outline"
            size="sm"
            onClick={() => {
              openStartConsensusModal(
                popup,
                artwork.title,
                totalOracles,
                async () => {
                  try {
                    await handleStartConsensus(artwork.id);
                    popup.close();
                  } catch (error) {
                    console.error('Error starting consensus:', error);
                  }
                },
              );
            }}
          >
            Oracle's Consensus
          </Button>
        )}
        {!artwork.is_certified &&
          artwork.has_consensus &&
          !artwork.is_voted &&
          isOracle &&
          original && (
            <Button
              variant="outline"
              size="sm"
              onClick={() => {
                openVoteModal(
                  popup,
                  artwork.title,
                  original?.image,
                  async (voteType, description) => {
                    try {
                      await handleVote(artwork.id, description, voteType);
                      popup.close();
                    } catch (error) {
                      console.error('Error voting for artwork:', error);
                    }
                  },
                );
              }}
            >
              Vote for Certification
            </Button>
          )}
        {certificate && (
          <Button
            variant="outline"
            size="sm"
            onClick={() => openCertificationModal(popup, certificate)}
          >
            View Certificate
          </Button>
        )}
      </div>
    </div>
  );
}

export default function Artworks({ spaceId }: { spaceId: number }) {
  const { data: userInfo } = useUserInfo();
  const { data: dagit } = useDagitBySpaceId(spaceId);
  const { artworks, insertArtwork, insertedArtworks } = useDagitStore();

  const popup = usePopup();

  const isEdit = useEditCoordinatorStore().isEdit;
  const {
    startConsensus: { mutateAsync: startConsensus },
    voteConsensus: { mutateAsync: voteArtwork },
  } = useDagitByIdMutation(spaceId);

  // if (!artworks || artworks.length === 0) {
  //   return null;
  // }

  const handleAddArtwork = () => {
    openCreateArtworkModal(popup, async (title, description, file) => {
      insertArtwork(title, description, file);
    });
  };

  const handleStartConsensus = async (artworkId: number) => {
    await startConsensus(artworkId);
  };

  const handleVote = async (
    artworkId: number,
    description: string | null,
    voteType: ConsensusVoteType,
  ) => {
    await voteArtwork({ artworkId, description, voteType });
  };
  return (
    <div className="flex flex-col gap-10">
      <div className="flex flex-row justify-between items-center">
        <h2 className="text-xl font-bold ">
          Total Artworks :{' '}
          {(artworks.length + insertedArtworks.length).toLocaleString()}
        </h2>
        {isEdit && (
          <Button variant="rounded_primary" onClick={handleAddArtwork}>
            Add Artwork
          </Button>
        )}
      </div>
      <div className="grid grid-cols-1 desktop:grid-cols-2 gap-4">
        {/* FIXME: Use Infinite Scroll with InfiniteQuery */}
        {artworks.slice(0, 10).map((artwork) => (
          <ArtworkViewer
            artwork={artwork}
            key={artwork.id}
            isOracle={dagit.is_oracle}
            totalOracles={dagit.oracles.length}
            handleStartConsensus={handleStartConsensus}
            handleVote={handleVote}
            isOwner={userInfo?.id === artwork.owner_id}
          />
        ))}
        {insertedArtworks.map((artwork) => (
          <ArtworkViewer artwork={artwork} key={artwork.id} isTemporary />
        ))}
      </div>
    </div>
  );
}
