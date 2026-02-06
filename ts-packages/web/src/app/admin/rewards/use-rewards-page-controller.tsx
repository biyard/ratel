import { useState, useEffect } from 'react';
import { useUserInfo } from '@/hooks/use-user-info';
import { useNavigate } from 'react-router';
import { route } from '@/route';
import {
  UpdateRewardRequest,
  useUpdateRewardMutation,
} from './hooks/use-update-reward-mutation';
import {
  Reward,
  useRewards,
} from '@/features/spaces/rewards/hooks/use-rewards';
import { useCreateRewardMutation } from './hooks/use-create-reward-mutation';
import { UserType } from '@/lib/api/ratel/users.v3';

export class RewardsPageController {
  constructor(
    public rewards: Reward[],
    public isLoading: boolean,
    public error: Error | null,
    public isFormOpen: boolean,
    public editingReward: Reward | null,
    public openForm: (reward?: Reward) => void,
    public closeForm: () => void,
    public handleCreateReward: (request: UpdateRewardRequest) => Promise<void>,
    public handleUpdateReward: (request: UpdateRewardRequest) => Promise<void>,
    public isSubmitting: boolean,
  ) {}
}

export function useRewardsPageController() {
  const { data: user, isLoading: userLoading } = useUserInfo();
  const navigate = useNavigate();
  const isAdmin = user?.user_type === UserType.Admin;
  const { data: rewards = [], isLoading, error } = useRewards();

  const createReward = useCreateRewardMutation();
  const updateReward = useUpdateRewardMutation();

  const [isFormOpen, setIsFormOpen] = useState(false);
  const [editingReward, setEditingReward] = useState<Reward | null>(null);

  useEffect(() => {
    if (!userLoading && !isAdmin) {
      navigate(route.home());
    }
  }, [isAdmin, userLoading, navigate]);

  const openForm = (reward?: Reward) => {
    setEditingReward(reward || null);
    setIsFormOpen(true);
  };

  const closeForm = () => {
    setIsFormOpen(false);
    setEditingReward(null);
  };

  const handleCreateReward = async (request: UpdateRewardRequest) => {
    try {
      await createReward.mutateAsync(request);
      closeForm();
    } catch (error) {
      console.error('Failed to create reward:', error);
      throw error;
    }
  };

  const handleUpdateReward = async (request: UpdateRewardRequest) => {
    try {
      await updateReward.mutateAsync(request);
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
    createReward.isPending || updateReward.isPending,
  );
}
