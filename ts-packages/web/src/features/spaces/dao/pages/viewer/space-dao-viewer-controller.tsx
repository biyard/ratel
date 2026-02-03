import { useEffect, useMemo, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import { SpaceDaoResponse } from '@/features/spaces/dao/hooks/use-space-dao';
import { State } from '@/types/state';
import {
  SpaceDaoRewardListResponse,
  useSpaceDaoReward,
} from '@/features/spaces/dao/hooks/use-space-dao-reward';
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
    public chainRecipientCount: State<string | null>,
    public rewardBookmark: State<string | null>,
    public rewardHistory: State<(string | null)[]>,
    public reward: SpaceDaoRewardListResponse | undefined,
    public rewardLoading: boolean,
    public isDistributingPage: State<boolean>,
  ) {}

  get canDistributeReward() {
    return this.space?.isAdmin?.() ?? false;
  }

  get canPrevReward() {
    if (this.space?.isFinished) {
      return false;
    }
    return this.rewardHistory.get().length > 0;
  }

  get canNextReward() {
    if (this.space?.isFinished) {
      return false;
    }
    return Boolean(this.reward?.bookmark);
  }

  get visibleRewardRecipients() {
    return this.reward?.items ?? [];
  }

  get rewardRecipients() {
    return this.reward;
  }

  get rewardRecipientsLoading() {
    return this.rewardLoading;
  }

  fetchRecipientCount = async () => {
    if (!this.provider || !this.dao?.contract_address) {
      return;
    }
    try {
      const service = new SpaceDaoService(this.provider);
      const count = await service.getRewardRecipientCount(
        this.dao.contract_address,
      );
      this.chainRecipientCount.set(String(count));
    } catch (error) {
      console.error('Failed to fetch reward recipient count:', error);
      this.chainRecipientCount.set(null);
    }
  };

  handleNextReward = () => {
    const next = this.reward?.bookmark ?? null;
    if (!next) return;
    const history = [...this.rewardHistory.get()];
    history.push(this.rewardBookmark.get());
    this.rewardHistory.set(history);
    this.rewardBookmark.set(next);
  };

  handlePrevReward = () => {
    const history = [...this.rewardHistory.get()];
    if (history.length === 0) return;
    const prev = history.pop() ?? null;
    this.rewardHistory.set(history);
    this.rewardBookmark.set(prev);
  };
}

export function useSpaceDaoViewerController(
  spacePk: string,
  dao?: SpaceDaoResponse | null,
) {
  const { data: space } = useSpaceById(spacePk);
  const { t } = useTranslation('SpaceDaoEditor');
  const chainRecipientCount = useState<string | null>(null);
  const rewardBookmark = useState<string | null>(null);
  const rewardHistory = useState<(string | null)[]>([]);
  const isDistributingPage = useState(false);
  const { data: reward, isLoading: rewardLoading } = useSpaceDaoReward(
    spacePk,
    rewardBookmark[0],
    50,
    Boolean(dao?.contract_address),
  );
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
    new State(chainRecipientCount),
    new State(rewardBookmark),
    new State(rewardHistory),
    reward,
    rewardLoading,
    new State(isDistributingPage),
  );

  useEffect(() => {
    void ctrl.fetchRecipientCount();
  }, [dao?.contract_address, provider]);

  return ctrl;
}
