import { useState, useEffect } from 'react';
import { useRewardsData } from './use-rewards-data';
import { useUserInfo } from '@/hooks/use-user-info';
import { useNavigate } from 'react-router';
import { route } from '@/route';
import { UpdateRewardRequest } from './hooks/use-update-reward-mutation';
import { RewardResponse } from '@/features/spaces/rewards/hooks/use-rewards';

const USER_TYPE_ADMIN = 98;

export class RewardsPageController {
  constructor(
    public rewards: RewardResponse[],
    public isLoading: boolean,
    public error: Error | null,
    public isFormOpen: boolean,
    public editingReward: RewardResponse | null,
    public openForm: (reward?: RewardResponse) => void,
    public closeForm: () => void,
    public handleCreateReward: (request: UpdateRewardRequest) => Promise<void>,
    public handleUpdateReward: (request: UpdateRewardRequest) => Promise<void>,
    public isSubmitting: boolean,
  ) {}
}

export function useRewardsPageController() {
  const { data: user, isLoading: userLoading } = useUserInfo();
  const navigate = useNavigate();
  const isAdmin = user?.user_type === USER_TYPE_ADMIN;

  const {
    rewards,
    isLoading,
    error,
    createReward,
    updateReward,
    isCreating,
    isUpdating,
  } = useRewardsData();

  const [isFormOpen, setIsFormOpen] = useState(false);
  const [editingReward, setEditingReward] = useState<RewardResponse | null>(
    null,
  );

  useEffect(() => {
    if (!userLoading && !isAdmin) {
      navigate(route.home());
    }
  }, [isAdmin, userLoading, navigate]);

  const openForm = (reward?: RewardResponse) => {
    setEditingReward(reward || null);
    setIsFormOpen(true);
  };

  const closeForm = () => {
    setIsFormOpen(false);
    setEditingReward(null);
  };

  const handleCreateReward = async (request: UpdateRewardRequest) => {
    try {
      await createReward(request);
      closeForm();
    } catch (error) {
      console.error('Failed to create reward:', error);
      throw error;
    }
  };

  const handleUpdateReward = async (request: UpdateRewardRequest) => {
    try {
      await updateReward(request);
      closeForm();
    } catch (error) {
      console.error('Failed to update reward:', error);
      throw error;
    }
  };

  return new RewardsPageController(
    rewards,
    isLoading,
    error,
    isFormOpen,
    editingReward,
    openForm,
    closeForm,
    handleCreateReward,
    handleUpdateReward,
    isCreating || isUpdating,
  );
}
