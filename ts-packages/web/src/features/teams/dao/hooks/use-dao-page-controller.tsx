import { useState, useMemo } from 'react';
import { useQueryClient } from '@tanstack/react-query';
import { useDaoData } from './use-dao-data';
import { TeamDaoService } from '@/contracts/TeamDaoService';
import {
  getKaiaSigner,
  KaiaWalletError,
} from '@/lib/service/kaia-wallet-service';
import { useUpdateTeam } from '@/features/teams/hooks/use-update-team';
import { teamKeys } from '@/constants';
import { config } from '@/config';
import { showErrorToast, showSuccessToast, showInfoToast } from '@/lib/toast';

export function useDaoPageController(username: string) {
  const { team, eligibleAdmins, permissions } = useDaoData(username, true);
  const [isPopupOpen, setIsPopupOpen] = useState(false);
  const [isRegistering, setIsRegistering] = useState(false);
  const updateTeamMutation = useUpdateTeam();
  const queryClient = useQueryClient();

  const canRegisterDao = useMemo(() => {
    return eligibleAdmins.length >= 3 && permissions?.isAdmin();
  }, [eligibleAdmins.length, permissions]);

  const blockExplorerUrl = useMemo(() => {
    if (!team.dao_address) return null;
    return `${config.block_explorer_url}/account/${team.dao_address}`;
  }, [team.dao_address]);

  const handleOpenRegistrationPopup = () => {
    if (!canRegisterDao) {
      showErrorToast(
        'Insufficient eligible admins. Need at least 3 team admins with EVM addresses.',
      );
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
      showErrorToast('At least 3 admins must be selected');
      return;
    }

    setIsRegistering(true);

    try {
      showInfoToast('Connecting to Kaia network...');
      const signer = await getKaiaSigner(
        config.env === 'prod' ? 'mainnet' : 'testnet',
      );
      const provider = signer.provider;

      showInfoToast('Creating DAO on blockchain...');
      const daoService = new TeamDaoService(provider);
      await daoService.connectWallet();

      const result = await daoService.createDAO(selectedAdminAddresses);

      showInfoToast('Saving DAO address...');
      await updateTeamMutation.mutateAsync({
        teamPk: team.pk,
        request: {
          dao_address: result.daoAddress,
        },
      });

      await queryClient.invalidateQueries({ queryKey: teamKeys.all });

      showSuccessToast(
        `DAO registered successfully! Transaction: ${result.transactionHash.slice(0, 10)}...`,
      );

      setIsPopupOpen(false);
    } catch (error) {
      console.error('Failed to register DAO:', error);

      if (error instanceof KaiaWalletError) {
        if (error.code === 'USER_REJECTED') {
          showErrorToast('Transaction cancelled: You rejected the transaction');
        } else if (error.code === 'METAMASK_NOT_INSTALLED') {
          showErrorToast(
            'MetaMask not installed. Please install MetaMask to continue',
          );
        } else {
          showErrorToast(`Wallet error: ${error.message}`);
        }
      } else if (error instanceof Error) {
        showErrorToast(`Failed to register DAO: ${error.message}`);
      } else {
        showErrorToast('Failed to register DAO: An unknown error occurred');
      }
    } finally {
      setIsRegistering(false);
    }
  };

  return {
    team,
    permissions,
    eligibleAdmins,
    eligibleAdminsCount: eligibleAdmins.length,
    canRegisterDao,
    isRegistering,
    isPopupOpen,
    blockExplorerUrl,
    handleOpenRegistrationPopup,
    handleClosePopup,
    handleRegisterDao,
  };
}
