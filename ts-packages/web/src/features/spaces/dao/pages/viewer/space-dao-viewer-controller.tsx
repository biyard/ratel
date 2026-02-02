import { useEffect, useMemo, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import { SpaceDaoResponse } from '@/features/spaces/dao/hooks/use-space-dao';
import { State } from '@/types/state';
import {
  SpaceDaoSampleListResponse,
  useSpaceDaoSamples,
} from '@/features/spaces/dao/hooks/use-space-dao-samples';
import { useSpaceDaoCandidates } from '@/features/spaces/dao/hooks/use-space-dao-candidates';
import { SpaceDaoService } from '@/contracts/SpaceDaoService';
import { config } from '@/config';
import { ethers } from 'ethers';

export class SpaceDaoViewerController {
  constructor(
    public spacePk: string,
    public space: Space | undefined,
    public dao: SpaceDaoResponse | null | undefined,
    public t: TFunction<'SpaceDaoEditor', undefined>,
    public provider: ethers.JsonRpcProvider | null,
    public chainSamplingCount: State<string | null>,
    public sampleBookmark: State<string | null>,
    public sampleHistory: State<(string | null)[]>,
    public samples: SpaceDaoSampleListResponse | undefined,
    public samplesLoading: boolean,
    public isDistributingPage: State<boolean>,
  ) {}

  get canDistributeReward() {
    return this.space?.isAdmin?.() ?? false;
  }

  get canPrevSample() {
    if (this.space?.isFinished) {
      return false;
    }
    return this.sampleHistory.get().length > 0;
  }

  get canNextSample() {
    if (this.space?.isFinished) {
      return false;
    }
    return Boolean(this.samples?.bookmark);
  }

  get visibleSamples() {
    return this.samples?.items ?? [];
  }

  fetchSamplingCount = async () => {
    if (!this.provider || !this.dao?.contract_address) {
      return;
    }
    try {
      const service = new SpaceDaoService(this.provider);
      const count = await service.getSamplingCount(this.dao.contract_address);
      this.chainSamplingCount.set(String(count));
    } catch (error) {
      console.error('Failed to fetch sampling count:', error);
      this.chainSamplingCount.set(null);
    }
  };

  handleNextSample = () => {
    const next = this.samples?.bookmark ?? null;
    if (!next) return;
    const history = [...this.sampleHistory.get()];
    history.push(this.sampleBookmark.get());
    this.sampleHistory.set(history);
    this.sampleBookmark.set(next);
  };

  handlePrevSample = () => {
    const history = [...this.sampleHistory.get()];
    if (history.length === 0) return;
    const prev = history.pop() ?? null;
    this.sampleHistory.set(history);
    this.sampleBookmark.set(prev);
  };
}

export function useSpaceDaoViewerController(
  spacePk: string,
  dao?: SpaceDaoResponse | null,
) {
  const { data: space } = useSpaceById(spacePk);
  const { t } = useTranslation('SpaceDaoEditor');
  const chainSamplingCount = useState<string | null>(null);
  const sampleBookmark = useState<string | null>(null);
  const sampleHistory = useState<(string | null)[]>([]);
  const isDistributingPage = useState(false);
  const sampledAddresses = useState<string[]>([]);
  const sampledLoading = useState(false);
  const isFinished = Boolean(space?.isFinished);
  const { data: samples, isLoading: samplesLoading } = useSpaceDaoSamples(
    spacePk,
    sampleBookmark[0],
    50,
    Boolean(dao?.contract_address) && !isFinished,
  );
  const { data: candidates, isLoading: candidatesLoading } =
    useSpaceDaoCandidates(
      spacePk,
      Boolean(dao?.contract_address) && isFinished,
    );
  const sampledCandidates: SpaceDaoSampleListResponse | undefined =
    useMemo(() => {
      if (!candidates?.candidates || sampledAddresses[0].length === 0) {
        return undefined;
      }
      const addressSet = new Set(
        sampledAddresses[0].map((addr) => addr.toLowerCase()),
      );
      const items = candidates.candidates
        .filter((item) => addressSet.has(item.evm_address.toLowerCase()))
        .map((item) => ({
          pk: item.user_pk,
          sk: `SAMPLED#${item.evm_address}`,
          user_pk: item.user_pk,
          username: item.username,
          display_name: item.display_name,
          profile_url: item.profile_url,
          evm_address: item.evm_address,
          reward_distributed: false,
        }));
      return {
        items,
        bookmark: null,
      };
    }, [candidates, sampledAddresses[0]]);
  const effectiveSamples = isFinished ? sampledCandidates : samples;
  const effectiveSamplesLoading = isFinished
    ? candidatesLoading || sampledLoading[0]
    : samplesLoading;
  const provider = useMemo(() => {
    if (!config.rpc_url) {
      return null;
    }
    return new ethers.JsonRpcProvider(config.rpc_url);
  }, []);

  const ctrl = new SpaceDaoViewerController(
    spacePk,
    space,
    dao,
    t,
    provider,
    new State(chainSamplingCount),
    new State(sampleBookmark),
    new State(sampleHistory),
    effectiveSamples,
    effectiveSamplesLoading,
    new State(isDistributingPage),
  );

  useEffect(() => {
    void ctrl.fetchSamplingCount();
  }, [dao?.contract_address, provider]);

  useEffect(() => {
    if (!isFinished || !dao?.contract_address || !provider) {
      return;
    }

    let cancelled = false;
    const fetchSampled = async () => {
      sampledLoading[1](true);
      try {
        const service = new SpaceDaoService(provider);
        const addresses = await service.getSampledAddresses(
          dao.contract_address,
        );
        if (!cancelled) {
          sampledAddresses[1](addresses ?? []);
        }
      } catch (error) {
        console.error('Failed to fetch sampled addresses:', error);
        if (!cancelled) {
          sampledAddresses[1]([]);
        }
      } finally {
        if (!cancelled) {
          sampledLoading[1](false);
        }
      }
    };

    void fetchSampled();
    return () => {
      cancelled = true;
    };
  }, [dao?.contract_address, isFinished, provider]);

  return ctrl;
}
