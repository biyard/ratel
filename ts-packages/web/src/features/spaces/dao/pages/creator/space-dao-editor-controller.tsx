import { useEffect, useMemo, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import { useDaoData } from '@/features/teams/dao/hooks/use-dao-data';
import { UserType } from '@/lib/api/ratel/users.v3';
import { showErrorToast, showInfoToast, showSuccessToast } from '@/lib/toast';
import {
  getKaiaSigner,
  KaiaWalletError,
} from '@/lib/service/kaia-wallet-service';
import { SpaceDaoService } from '@/contracts/SpaceDaoService';
import { config } from '@/config';
import { useCreateSpaceDaoMutation } from '@/features/spaces/dao/hooks/use-create-space-dao-mutation';
import { SpaceDaoResponse } from '@/features/spaces/dao/hooks/use-space-dao';
import { ethers } from 'ethers';
import { State } from '@/types/state';
import {
  SpaceDaoSampleListResponse,
  useSpaceDaoSamples,
} from '@/features/spaces/dao/hooks/use-space-dao-samples';
import { useUpdateSpaceDaoSamplesMutation } from '@/features/spaces/dao/hooks/use-update-space-dao-samples-mutation';
import { useSpaceDaoCandidates } from '@/features/spaces/dao/hooks/use-space-dao-candidates';

export class SpaceDaoEditorController {
  constructor(
    public spacePk: string,
    public space: Space | undefined,
    public dao: SpaceDaoResponse | null | undefined,
    public eligibleAdmins: ReturnType<typeof useDaoData>['eligibleAdmins'],
    public permissions: ReturnType<typeof useDaoData>['permissions'],
    public t: TFunction<'SpaceDaoEditor', undefined>,
    public provider: ethers.JsonRpcProvider | null,
    public adminAddresses: State<string>,
    public samplingCount: State<string>,
    public chainSamplingCount: State<string | null>,
    public isPopupOpen: State<boolean>,
    public isRegistering: State<boolean>,
    public isUpdating: State<boolean>,
    public sampleBookmark: State<string | null>,
    public sampleHistory: State<(string | null)[]>,
    public samples: SpaceDaoSampleListResponse | undefined,
    public samplesLoading: boolean,
    public isDistributingPage: State<boolean>,
    public createSpaceDaoMutation: ReturnType<typeof useCreateSpaceDaoMutation>,
    public updateSamplesMutation: ReturnType<
      typeof useUpdateSpaceDaoSamplesMutation
    >,
  ) {}

  get isTeamSpace() {
    return (this.space?.authorType ?? null) === UserType.Team;
  }

  get canRegisterDao() {
    return this.eligibleAdmins.length >= 3 && this.permissions?.isAdmin();
  }

  get canSubmitInputs() {
    const sampling = Number(this.samplingCount.get());
    return Number.isFinite(sampling) && sampling > 0;
  }

  get canDistributeReward() {
    if (this.space?.isFinished) {
      return false;
    }
    return this.permissions?.isAdmin() ?? false;
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

  handleDistribute = async () => {
    if (!this.canDistributeReward) return;
    const dao = this.dao;
    if (!dao?.contract_address) return;

    const candidates = this.visibleSamples.filter(
      (item) => !item.reward_distributed,
    );
    if (candidates.length === 0) {
      return;
    }
    const target = candidates.slice(0, 2);
    const recipients = target.map((item) => item.evm_address);
    const sampleSks = target.map((item) => item.sk);

    this.isDistributingPage.set(true);
    try {
      showInfoToast(this.t('toast_connecting_wallet'));
      const signer = await getKaiaSigner(
        config.env === 'prod' ? 'mainnet' : 'testnet',
      );
      const provider = signer.provider;
      const daoService = new SpaceDaoService(provider);
      await daoService.connectWallet();
      await daoService.spaceDistributeWithdrawal(
        dao.contract_address,
        recipients,
      );

      await this.updateSamplesMutation.mutateAsync({
        spacePk: this.spacePk,
        sampleSks,
        rewardDistributed: true,
      });

      showSuccessToast(this.t('toast_reward_distributed'));
    } catch (error) {
      console.error('Failed to distribute reward:', error);
      if (error instanceof Error) {
        showErrorToast(
          this.t('error_reward_distribute_failed', { message: error.message }),
        );
      } else {
        showErrorToast(this.t('error_register_failed_unknown'));
      }
    } finally {
      this.isDistributingPage.set(false);
    }
  };

  handleOpenRegistrationPopup = () => {
    if (!this.canRegisterDao) {
      showErrorToast(this.t('error_insufficient_admins'));
      return;
    }
    if (!this.canSubmitInputs) {
      showErrorToast(this.t('error_missing_inputs'));
      return;
    }
    this.isPopupOpen.set(true);
  };

  handleClosePopup = () => {
    if (!this.isRegistering.get()) {
      this.isPopupOpen.set(false);
    }
  };

  handleRegisterDao = async (selectedAdminAddresses: string[]) => {
    if (selectedAdminAddresses.length < 3) {
      showErrorToast(this.t('error_invalid_admin_selection'));
      return;
    }

    this.isRegistering.set(true);

    try {
      showInfoToast(this.t('toast_connecting_wallet'));
      const signer = await getKaiaSigner(
        config.env === 'prod' ? 'mainnet' : 'testnet',
      );
      const provider = signer.provider;

      showInfoToast(this.t('toast_creating_dao'));
      const daoService = new SpaceDaoService(provider);
      await daoService.connectWallet();

      const sampling = Number(this.samplingCount.get());
      const result = await daoService.createSpaceDAO(
        selectedAdminAddresses,
        sampling,
      );

      await this.createSpaceDaoMutation.mutateAsync({
        spacePk: this.spacePk,
        req: {
          contract_address: result.daoAddress,
          deploy_block: result.deployBlock,
        },
      });

      this.chainSamplingCount.set(String(sampling));
      showSuccessToast(this.t('toast_registered'));
      this.isPopupOpen.set(false);
    } catch (error) {
      console.error('Failed to register Space DAO:', error);

      if (error instanceof KaiaWalletError) {
        if (error.code === 'USER_REJECTED') {
          showErrorToast(this.t('error_wallet_rejected'));
        } else if (error.code === 'METAMASK_NOT_INSTALLED') {
          showErrorToast(this.t('error_wallet_not_installed'));
        } else {
          showErrorToast(
            this.t('error_wallet_generic', { message: error.message }),
          );
        }
      } else if (error instanceof Error) {
        showErrorToast(
          this.t('error_register_failed', { message: error.message }),
        );
      } else {
        showErrorToast(this.t('error_register_failed_unknown'));
      }
    } finally {
      this.isRegistering.set(false);
    }
  };

  handleUpdateDao = async (samplingCount: string) => {
    const dao = this.dao;
    const sampling = Number(samplingCount);

    if (!dao?.contract_address) {
      showErrorToast(this.t('error_register_failed_unknown'));
      return;
    }
    if (!Number.isFinite(sampling) || sampling <= 0) {
      showErrorToast(this.t('error_invalid_sampling_count'));
      return;
    }

    this.isUpdating.set(true);
    try {
      showInfoToast(this.t('toast_connecting_wallet'));
      const signer = await getKaiaSigner(
        config.env === 'prod' ? 'mainnet' : 'testnet',
      );
      const provider = signer.provider;

      const daoService = new SpaceDaoService(provider);
      await daoService.connectWallet();
      await daoService.setSamplingCount(dao.contract_address, sampling);
      this.chainSamplingCount.set(String(sampling));
      showSuccessToast(this.t('toast_updated'));
    } catch (error) {
      console.error('Failed to update Space DAO:', error);
      if (error instanceof Error) {
        showErrorToast(
          this.t('error_register_failed', { message: error.message }),
        );
      } else {
        showErrorToast(this.t('error_register_failed_unknown'));
      }
    } finally {
      this.isUpdating.set(false);
    }
  };
}

export function useSpaceDaoEditorController(
  spacePk: string,
  dao?: SpaceDaoResponse | null,
) {
  const { data: space } = useSpaceById(spacePk);
  const { t } = useTranslation('SpaceDaoEditor');
  const adminAddresses = useState('');
  const samplingCount = useState('');
  const chainSamplingCount = useState<string | null>(null);
  const isPopupOpen = useState(false);
  const isRegistering = useState(false);
  const isUpdating = useState(false);
  const sampleBookmark = useState<string | null>(null);
  const sampleHistory = useState<(string | null)[]>([]);
  const isDistributingPage = useState(false);
  const sampledAddresses = useState<string[]>([]);
  const sampledLoading = useState(false);
  const createSpaceDaoMutation = useCreateSpaceDaoMutation();
  const updateSamplesMutation = useUpdateSpaceDaoSamplesMutation();
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

  const teamUsername = space?.authorUsername ?? '';
  const { eligibleAdmins, permissions } = useDaoData(teamUsername);

  const ctrl = new SpaceDaoEditorController(
    spacePk,
    space,
    dao,
    eligibleAdmins,
    permissions,
    t,
    provider,
    new State(adminAddresses),
    new State(samplingCount),
    new State(chainSamplingCount),
    new State(isPopupOpen),
    new State(isRegistering),
    new State(isUpdating),
    new State(sampleBookmark),
    new State(sampleHistory),
    effectiveSamples,
    effectiveSamplesLoading,
    new State(isDistributingPage),
    createSpaceDaoMutation,
    updateSamplesMutation,
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

    fetchSampled();
    return () => {
      cancelled = true;
    };
  }, [dao?.contract_address, isFinished, provider]);

  return ctrl;
}
