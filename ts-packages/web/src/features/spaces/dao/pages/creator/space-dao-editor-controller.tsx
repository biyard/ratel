import { useMemo, useState } from 'react';
import { useTranslation } from 'react-i18next';
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

export class SpaceDaoEditorController {
  constructor(
    public spacePk: string,
    public space: Space | undefined,
    public adminAddresses: string,
    public setAdminAddresses: (value: string) => void,
    public samplingCount: string,
    public setSamplingCount: (value: string) => void,
    public rewardAmount: string,
    public setRewardAmount: (value: string) => void,
    public eligibleAdmins: ReturnType<typeof useDaoData>['eligibleAdmins'],
    public permissions: ReturnType<typeof useDaoData>['permissions'],
    public isTeamSpace: boolean,
    public isPopupOpen: boolean,
    public isRegistering: boolean,
    public canRegisterDao: boolean,
    public canSubmitInputs: boolean,
    public handleOpenRegistrationPopup: () => void,
    public handleClosePopup: () => void,
    public handleRegisterDao: (
      selectedAdminAddresses: string[],
    ) => Promise<void>,
  ) {}
}

export function useSpaceDaoEditorController(spacePk: string) {
  const { data: space } = useSpaceById(spacePk);
  const { t } = useTranslation('SpaceDaoEditor');
  const [adminAddresses, setAdminAddresses] = useState('');
  const [samplingCount, setSamplingCount] = useState('');
  const [rewardAmount, setRewardAmount] = useState('');
  const [isPopupOpen, setIsPopupOpen] = useState(false);
  const [isRegistering, setIsRegistering] = useState(false);

  const isTeamSpace = (space?.authorType ?? null) === UserType.Team;
  const teamUsername = space?.authorUsername ?? '';
  const { eligibleAdmins, permissions } = useDaoData(teamUsername);

  const canRegisterDao = useMemo(() => {
    return eligibleAdmins.length >= 3 && permissions?.isAdmin();
  }, [eligibleAdmins.length, permissions]);

  const canSubmitInputs = useMemo(() => {
    const sampling = Number(samplingCount);
    const reward = Number(rewardAmount);
    return (
      Number.isFinite(sampling) &&
      sampling > 0 &&
      Number.isFinite(reward) &&
      reward > 0
    );
  }, [samplingCount, rewardAmount]);

  const handleOpenRegistrationPopup = () => {
    if (!canRegisterDao) {
      showErrorToast(t('error_insufficient_admins'));
      return;
    }
    if (!canSubmitInputs) {
      showErrorToast(t('error_missing_inputs'));
      return;
    }
    setIsPopupOpen(true);
  };

  const handleClosePopup = () => {
    if (!isRegistering) {
      setIsPopupOpen(false);
    }
  };

  const handleRegisterDao = async (selectedAdminAddresses: string[]) => {
    if (selectedAdminAddresses.length < 3) {
      showErrorToast(t('error_invalid_admin_selection'));
      return;
    }

    if (!rewardAmount || Number(rewardAmount) <= 0) {
      showErrorToast(t('error_invalid_reward_amount'));
      return;
    }

    setIsRegistering(true);

    try {
      showInfoToast(t('toast_connecting_wallet'));
      const signer = await getKaiaSigner(
        config.env === 'prod' ? 'mainnet' : 'testnet',
      );
      const provider = signer.provider;

      showInfoToast(t('toast_creating_dao'));
      const blockchainService = new BlockchainService(provider);
      await blockchainService.connectWallet();

      const result = await blockchainService.createSpaceDAO(
        selectedAdminAddresses,
        config.usdt_address,
        rewardAmount,
      );

      showSuccessToast(t('toast_registered'));

      console.log('transaction hash: ', result);

      setIsPopupOpen(false);
    } catch (error) {
      console.error('Failed to register Space DAO:', error);

      if (error instanceof KaiaWalletError) {
        if (error.code === 'USER_REJECTED') {
          showErrorToast(t('error_wallet_rejected'));
        } else if (error.code === 'METAMASK_NOT_INSTALLED') {
          showErrorToast(t('error_wallet_not_installed'));
        } else {
          showErrorToast(t('error_wallet_generic', { message: error.message }));
        }
      } else if (error instanceof Error) {
        showErrorToast(t('error_register_failed', { message: error.message }));
      } else {
        showErrorToast(t('error_register_failed_unknown'));
      }
    } finally {
      setIsRegistering(false);
    }
  };

  return new SpaceDaoEditorController(
    spacePk,
    space,
    adminAddresses,
    setAdminAddresses,
    samplingCount,
    setSamplingCount,
    rewardAmount,
    setRewardAmount,
    eligibleAdmins,
    permissions,
    isTeamSpace,
    isPopupOpen,
    isRegistering,
    canRegisterDao,
    canSubmitInputs,
    handleOpenRegistrationPopup,
    handleClosePopup,
    handleRegisterDao,
  );
}
