import { useEffect, useMemo, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import { SpaceDaoResponse } from '@/features/spaces/dao/hooks/use-space-dao';
import { State } from '@/types/state';
import {
  SpaceDaoSelectedListResponse,
  useSpaceDaoSelected,
} from '@/features/spaces/dao/hooks/use-space-dao-selected';
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
    public selectedBookmark: State<string | null>,
    public selectedHistory: State<(string | null)[]>,
    public selected: SpaceDaoSelectedListResponse | undefined,
    public selectedLoading: boolean,
    public isDistributingPage: State<boolean>,
  ) {}

  get canDistributeReward() {
    return this.space?.isAdmin?.() ?? false;
  }

  get canPrevSelected() {
    if (this.space?.isFinished) {
      return false;
    }
    return this.selectedHistory.get().length > 0;
  }

  get canNextSelected() {
    if (this.space?.isFinished) {
      return false;
    }
    return Boolean(this.selected?.bookmark);
  }

  get visibleSelected() {
    return this.selected?.items ?? [];
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

  handleNextSelected = () => {
    const next = this.selected?.bookmark ?? null;
    if (!next) return;
    const history = [...this.selectedHistory.get()];
    history.push(this.selectedBookmark.get());
    this.selectedHistory.set(history);
    this.selectedBookmark.set(next);
  };

  handlePrevSelected = () => {
    const history = [...this.selectedHistory.get()];
    if (history.length === 0) return;
    const prev = history.pop() ?? null;
    this.selectedHistory.set(history);
    this.selectedBookmark.set(prev);
  };
}

export function useSpaceDaoViewerController(
  spacePk: string,
  dao?: SpaceDaoResponse | null,
) {
  const { data: space } = useSpaceById(spacePk);
  const { t } = useTranslation('SpaceDaoEditor');
  const chainRecipientCount = useState<string | null>(null);
  const selectedBookmark = useState<string | null>(null);
  const selectedHistory = useState<(string | null)[]>([]);
  const isDistributingPage = useState(false);
  const { data: selected, isLoading: selectedLoading } = useSpaceDaoSelected(
    spacePk,
    selectedBookmark[0],
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
    new State(selectedBookmark),
    new State(selectedHistory),
    selected,
    selectedLoading,
    new State(isDistributingPage),
  );

  useEffect(() => {
    void ctrl.fetchRecipientCount();
  }, [dao?.contract_address, provider]);

  return ctrl;
}
