import { useState, useEffect } from 'react';
import { useRewardsData } from './use-rewards-data';
import { useUserInfo } from '@/hooks/use-user-info';
import { useNavigate } from 'react-router';
import { route } from '@/route';
import type {
  GlobalRewardResponse,
  UpdateGlobalRewardRequest,
} from '@/features/spaces/rewards/types';

const USER_TYPE_ADMIN = 98;

export class RewardsPageController {
  constructor(
    public rewards: GlobalRewardResponse[],
    public isLoading: boolean,
    public error: Error | null,
    public isFormOpen: boolean,
    public editingReward: GlobalRewardResponse | null,
    public openEditForm: (reward: GlobalRewardResponse) => void,
    public closeForm: () => void,
    public handleUpdateReward: (
      request: UpdateGlobalRewardRequest,
    ) => Promise<void>,
    public isSubmitting: boolean,
  ) {}
}

export function useRewardsPageController() {
  const { data: user, isLoading: userLoading } = useUserInfo();
  const navigate = useNavigate();
  const isAdmin = user?.user_type === USER_TYPE_ADMIN;

  const { rewards, isLoading, error, updateReward, isUpdating } =
    useRewardsData();

  const [isFormOpen, setIsFormOpen] = useState(false);
  const [editingReward, setEditingReward] =
    useState<GlobalRewardResponse | null>(null);

  useEffect(() => {
    if (!userLoading && !isAdmin) {
      navigate(route.home());
    }
  }, [isAdmin, userLoading, navigate]);

  const openEditForm = (reward: GlobalRewardResponse) => {
    setEditingReward(reward);
    setIsFormOpen(true);
  };

  const closeForm = () => {
    setIsFormOpen(false);
    setEditingReward(null);
  };

  const handleUpdateReward = async (request: UpdateGlobalRewardRequest) => {
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
    openEditForm,
    closeForm,
    handleUpdateReward,
    isUpdating,
  );
}
