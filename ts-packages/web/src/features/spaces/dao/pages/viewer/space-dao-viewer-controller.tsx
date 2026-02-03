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
    public rewardPageIndex: State<number>,
    public rewardPages: SpaceDaoRewardListResponse[] | undefined,
    public rewardLoading: boolean,
    public rewardHasNextPage: boolean,
    public rewardFetchingNextPage: boolean,
    public fetchNextRewardPage: () => Promise<unknown>,
    public isDistributingPage: State<boolean>,
  ) {}

  get canDistributeReward() {
    return this.space?.isAdmin?.() ?? false;
  }

  get canPrevReward() {
    return this.rewardPageIndex.get() > 0;
  }

  get canNextReward() {
    const index = this.rewardPageIndex.get();
    const pages = this.rewardPages ?? [];
    return index < pages.length - 1 || this.rewardHasNextPage;
  }

  get visibleRewardRecipients() {
    return this.rewardRecipients?.items ?? [];
  }

  get rewardRecipients() {
    const pages = this.rewardPages ?? [];
    const index = this.rewardPageIndex.get();
    return pages[index] ?? pages[0];
  }

  get rewardRecipientsLoading() {
    return this.rewardLoading || this.rewardFetchingNextPage;
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

  handleNextReward = async () => {
    const pages = this.rewardPages ?? [];
    const index = this.rewardPageIndex.get();
    if (index < pages.length - 1) {
      this.rewardPageIndex.set(index + 1);
      return;
    }
    if (!this.rewardHasNextPage) return;
    await this.fetchNextRewardPage();
    this.rewardPageIndex.set(index + 1);
  };

  handlePrevReward = () => {
    const index = this.rewardPageIndex.get();
    if (index <= 0) return;
    this.rewardPageIndex.set(index - 1);
  };
}

export function useSpaceDaoViewerController(
  spacePk: string,
  dao?: SpaceDaoResponse | null,
) {
  const { data: space } = useSpaceById(spacePk);
  const { t } = useTranslation('SpaceDaoEditor');
  const chainRecipientCount = useState<string | null>(null);
  const rewardPageIndex = useState(0);
  const isDistributingPage = useState(false);
  const {
    data: reward,
    isLoading: rewardLoading,
    fetchNextPage: fetchNextRewardPage,
    hasNextPage: rewardHasNextPage,
    isFetchingNextPage: rewardFetchingNextPage,
  } = useSpaceDaoReward(spacePk, 50, Boolean(dao?.contract_address));
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
    new State(rewardPageIndex),
    reward?.pages,
    rewardLoading,
    Boolean(rewardHasNextPage),
    rewardFetchingNextPage,
    fetchNextRewardPage,
    new State(isDistributingPage),
  );

  useEffect(() => {
    void ctrl.fetchRecipientCount();
  }, [dao?.contract_address, provider]);

  return ctrl;
}
