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
import { BlockchainService } from '@/contracts/BlockchainService';
import { config } from '@/config';
import { useCreateSpaceDaoMutation } from '@/features/spaces/dao/hooks/use-create-space-dao-mutation';
import { SpaceDaoResponse } from '@/features/spaces/dao/hooks/use-space-dao';
import { ethers } from 'ethers';
import { State } from '@/types/state';

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
    public rewardAmount: State<string>,
    public isPopupOpen: State<boolean>,
    public isRegistering: State<boolean>,
    public balance: State<string | null>,
    public balanceLoading: State<boolean>,
    public isDepositOpen: State<boolean>,
    public depositAmount: State<string>,
    public isDepositing: State<boolean>,
    public createSpaceDaoMutation: ReturnType<typeof useCreateSpaceDaoMutation>,
  ) {}

  get isTeamSpace() {
    return (this.space?.authorType ?? null) === UserType.Team;
  }

  get canRegisterDao() {
    return this.eligibleAdmins.length >= 3 && this.permissions?.isAdmin();
  }

  get canSubmitInputs() {
    const sampling = Number(this.samplingCount.get());
    const reward = Number(this.rewardAmount.get());
    return (
      Number.isFinite(sampling) &&
      sampling > 0 &&
      Number.isFinite(reward) &&
      reward > 0
    );
  }

  fetchBalance = async () => {
    if (!this.provider || !this.dao?.contract_address) {
      return;
    }

    this.balanceLoading.set(true);

    try {
      const service = new BlockchainService(this.provider);
      const raw = await service.getSpaceBalance(this.dao.contract_address);
      const formatted = ethers.formatUnits(raw, 6);
      this.balance.set(formatted);
    } catch (error) {
      console.error('Failed to fetch Space DAO balance:', error);
      this.balance.set(null);
    } finally {
      this.balanceLoading.set(false);
    }
  };

  handleOpenDeposit = () => {
    this.isDepositOpen.set(true);
  };

  handleCloseDeposit = () => {
    if (!this.isDepositing.get()) {
      this.isDepositOpen.set(false);
    }
  };

  handleDepositAmountChange = (value: string) => {
    this.depositAmount.set(value);
  };

  handleConfirmDeposit = async () => {
    const dao = this.dao;
    const amount = Number(this.depositAmount.get());
    if (!dao?.contract_address || !Number.isFinite(amount) || amount <= 0) {
      showErrorToast(this.t('error_invalid_deposit_amount'));
      return;
    }

    this.isDepositing.set(true);

    try {
      showInfoToast(this.t('toast_connecting_wallet'));
      const signer = await getKaiaSigner(
        config.env === 'prod' ? 'mainnet' : 'testnet',
      );
      const walletProvider = signer.provider;

      showInfoToast(this.t('toast_depositing'));
      const service = new BlockchainService(walletProvider);
      await service.connectWallet();
      await service.spaceDeposit(
        dao.contract_address,
        this.depositAmount.get(),
      );

      showSuccessToast(this.t('toast_deposit_success'));
      this.isDepositOpen.set(false);
      this.depositAmount.set('');
      await this.fetchBalance();
    } catch (error) {
      console.error('Failed to deposit to Space DAO:', error);

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
          this.t('error_deposit_failed', { message: error.message }),
        );
      } else {
        showErrorToast(this.t('error_register_failed_unknown'));
      }
    } finally {
      this.isDepositing.set(false);
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

    if (!this.rewardAmount.get() || Number(this.rewardAmount.get()) <= 0) {
      showErrorToast(this.t('error_invalid_reward_amount'));
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
      const blockchainService = new BlockchainService(provider);
      await blockchainService.connectWallet();

      const result = await blockchainService.createSpaceDAO(
        selectedAdminAddresses,
        config.usdt_address,
        this.rewardAmount.get(),
      );

      const sampling = Number(this.samplingCount.get());
      const reward = Number(this.rewardAmount.get());

      await this.createSpaceDaoMutation.mutateAsync({
        spacePk: this.spacePk,
        req: {
          contract_address: result.daoAddress,
          sampling_count: sampling,
          reward_amount: reward,
        },
      });

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
}

export function useSpaceDaoEditorController(
  spacePk: string,
  dao?: SpaceDaoResponse | null,
) {
  const { data: space } = useSpaceById(spacePk);
  const { t } = useTranslation('SpaceDaoEditor');
  const adminAddresses = useState('');
  const samplingCount = useState('');
  const rewardAmount = useState('');
  const isPopupOpen = useState(false);
  const isRegistering = useState(false);
  const balance = useState<string | null>(null);
  const balanceLoading = useState(false);
  const isDepositOpen = useState(false);
  const depositAmount = useState('');
  const isDepositing = useState(false);
  const createSpaceDaoMutation = useCreateSpaceDaoMutation();
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
    new State(rewardAmount),
    new State(isPopupOpen),
    new State(isRegistering),
    new State(balance),
    new State(balanceLoading),
    new State(isDepositOpen),
    new State(depositAmount),
    new State(isDepositing),
    createSpaceDaoMutation,
  );

  useEffect(() => {
    void ctrl.fetchBalance();
  }, [dao?.contract_address, provider]);

  return ctrl;
}
