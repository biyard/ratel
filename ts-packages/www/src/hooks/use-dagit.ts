import { QK_GET_DAGIT } from '@/constants';
import { apiFetch, FetchResponse } from '@/lib/api/apiFetch';
import { Dagit } from '@/lib/api/models/dagit';
import { ratelApi } from '@/lib/api/ratel_api';
import { config } from '@/config';

import {
  useMutation,
  useSuspenseQuery,
  UseSuspenseQueryResult,
} from '@tanstack/react-query';
import Artwork from '@/lib/api/models/artwork';
import { getQueryClient } from '@/providers/getQueryClient';
import { ConsensusVote, ConsensusVoteType } from '@/lib/api/models/consensus';
import { showSuccessToast } from '@/lib/toast';
import { FileInfo } from '@/lib/api/models/feeds';

export async function getDagitBySpaceId(
  spaceId: number,
): Promise<FetchResponse<Dagit | null>> {
  return apiFetch<Dagit | null>(
    `${config.api_url}${ratelApi.dagit.getDagitBySpaceId(spaceId)}`,
    {
      method: 'GET',
      headers: {
        'Content-Type': 'application/json',
      },
    },
  );
}

export async function addOracle(
  spaceId: number,
  oracleId: number,
): Promise<FetchResponse<Dagit | null>> {
  return apiFetch<Dagit | null>(
    `${config.api_url}${ratelApi.dagit.addOracle(spaceId)}`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ oracle_id: oracleId }),
    },
  );
}

export async function createArtwork(
  spaceId: number,
  title: string,
  image: FileInfo,
  description: string | null,
): Promise<FetchResponse<Artwork | null>> {
  return apiFetch<Artwork | null>(
    `${config.api_url}${ratelApi.dagit.createArtwork(spaceId)}`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        title,
        description,
        file: image,
      }),
    },
  );
}

export async function voteConsensus(
  spaceId: number,
  artworkId: number,
  description: string | null,
  voteType: ConsensusVoteType,
) {
  return apiFetch<ConsensusVote | null>(
    `${config.api_url}${ratelApi.dagit.voteConsensus(spaceId, artworkId)}`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        description,
        vote_type: voteType,
      }),
    },
  );
}

export async function startConsensus(spaceId: number, artworkId: number) {
  return apiFetch<ConsensusVote | null>(
    `${config.api_url}${ratelApi.dagit.startConsensus(spaceId)}`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        artwork_id: artworkId,
      }),
    },
  );
}

export const getQueryKey = (spaceId: number) => [QK_GET_DAGIT, spaceId];

export function getOption(spaceId: number) {
  return {
    queryKey: getQueryKey(spaceId),
    queryFn: async () => {
      const { data } = await getDagitBySpaceId(spaceId);
      if (!data) {
        throw new Error('Space not found');
      }
      return data;
    },
  };
}

export default function useDagitBySpaceId(
  spaceId: number,
): UseSuspenseQueryResult<Dagit> {
  const query = useSuspenseQuery(getOption(spaceId));
  return query;
}

export function useDagitByIdMutation(spaceId: number) {
  const queryClient = getQueryClient();
  const queryKey = getQueryKey(spaceId);

  const createArtworkMutation = useMutation({
    mutationFn: async ({
      title,
      image,
      description,
    }: {
      title: string;
      image: FileInfo;
      description: string | null;
    }) => {
      const { data } = await createArtwork(spaceId, title, image, description);
      if (!data) {
        throw new Error('Vote failed.');
      }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey });
      showSuccessToast('Artwork created successfully');
    },
  });

  const addOracleMutation = useMutation({
    mutationFn: async (oracleId: number) => {
      const { data } = await addOracle(spaceId, oracleId);
      if (!data) {
        throw new Error('Add oracle failed.');
      }
      return data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey });
      showSuccessToast('Oracle added successfully');
    },
  });

  const startConsensusMutation = useMutation({
    mutationFn: async (artworkId: number) => {
      const { data } = await startConsensus(spaceId, artworkId);
      if (!data) {
        throw new Error('Start consensus failed.');
      }
      return data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey });
      showSuccessToast('Consensus started successfully');
    },
  });

  const voteConsensusMutation = useMutation({
    mutationFn: async ({
      artworkId,
      description,
      voteType,
    }: {
      artworkId: number;
      description: string | null;
      voteType: ConsensusVoteType;
    }) => {
      const { data } = await voteConsensus(
        spaceId,
        artworkId,
        description,
        voteType,
      );
      if (!data) {
        throw new Error('Vote failed.');
      }
      return data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey });
      showSuccessToast('Consensus vote submitted successfully');
    },
  });

  return {
    addOracle: addOracleMutation,
    createArtwork: createArtworkMutation,
    startConsensus: startConsensusMutation,
    voteConsensus: voteConsensusMutation,
  };
}
