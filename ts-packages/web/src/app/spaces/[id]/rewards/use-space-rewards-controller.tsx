import { useState } from 'react';

import { State } from '@/types/state';
import { Space } from '@/features/spaces/types/space';
import { SpaceRewardResponse } from '@/features/spaces/rewards/types/space-reward-response';
import { ListRewardsResponse } from '@/features/spaces/rewards/types/list-rewards-response';
import { useCreateRewardMutation } from '@/features/spaces/rewards/hooks/use-create-reward-mutation';
import { useUpdateRewardMutation } from '@/features/spaces/rewards/hooks/use-update-reward-mutation';
import { useDeleteRewardMutation } from '@/features/spaces/rewards/hooks/use-delete-reward-mutation';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { logger } from '@/lib/logger';
import { CreateRewardRequest } from '@/features/spaces/rewards/types/create-reward-request';
import { UpdateRewardRequest } from '@/features/spaces/rewards/types/update-reward-request';
import { RewardsI18n, useRewardsI18n } from '@/features/spaces/rewards/i18n';

import { RewardType } from '@/features/spaces/rewards/types/reward-type';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { SpaceType } from '@/features/spaces/types/space-type';
import useSpaceRewards from '@/features/spaces/rewards/hooks/use-space-rewards';

export interface RewardFormData {
  rewardType?: RewardType;
  pollSk?: string;
  description?: string;
  credits: number;
}

export class SpaceRewardsController {
  constructor(
    public spacePk: string,
    public i18n: RewardsI18n,
    public space: Space,
    public rewards: ListRewardsResponse,
    public isPollSpace: boolean,

    public editingReward: State<SpaceRewardResponse | null>,
    public isModalOpen: State<boolean>,

    public createReward: ReturnType<typeof useCreateRewardMutation>,
    public updateReward: ReturnType<typeof useUpdateRewardMutation>,
    public deleteReward: ReturnType<typeof useDeleteRewardMutation>,
  ) {}

  openCreateModal = () => {
    this.editingReward.set(null);
    this.isModalOpen.set(true);
  };

  openEditModal = (reward: SpaceRewardResponse) => {
    this.editingReward.set(reward);
    this.isModalOpen.set(true);
  };

  closeModal = () => {
    this.editingReward.set(null);
    this.isModalOpen.set(false);
  };

  handleCreate = async (data: RewardFormData) => {
    if (!data.pollSk) return;

    try {
      const req: CreateRewardRequest = {
        reward: { poll_sk: data.pollSk },
        description: data.description,
        credits: data.credits,
      };

      await this.createReward.mutateAsync({
        spacePk: this.spacePk,
        req,
      });

      showSuccessToast(this.i18n.settings.create_success);
      this.closeModal();
    } catch (err) {
      logger.error('Failed to create reward', err);
      showErrorToast(this.i18n.settings.create_error);
    }
  };

  handleUpdate = async (data: RewardFormData, reward: SpaceRewardResponse) => {
    try {
      const req: UpdateRewardRequest = {
        sk: reward.sk,
        description: data.description,
        credits: data.credits,
      };

      await this.updateReward.mutateAsync({
        spacePk: this.spacePk,
        req,
      });

      showSuccessToast(this.i18n.settings.update_success);
      this.closeModal();
    } catch (err) {
      logger.error('Failed to update reward', err);
      showErrorToast(this.i18n.settings.update_error);
    }
  };

  handleDelete = async (reward: SpaceRewardResponse) => {
    try {
      await this.deleteReward.mutateAsync({
        spacePk: this.spacePk,
        req: { sk: reward.sk },
      });

      showSuccessToast(this.i18n.settings.delete_success);
    } catch (err) {
      logger.error('Failed to delete reward', err);
      showErrorToast(this.i18n.settings.delete_error);
    }
  };

  handleSubmit = async (data: RewardFormData) => {
    const editing = this.editingReward.get();
    if (editing) {
      await this.handleUpdate(data, editing);
    } else {
      await this.handleCreate(data);
    }
  };
}

export function useSpaceRewardsController(spacePk: string) {
  const i18n = useRewardsI18n();
  const { data: space } = useSpaceById(spacePk);
  const isPollSpace =
    space?.spaceType === SpaceType.Poll ||
    space?.spaceType === SpaceType.Deliberation;
  const { data: rewards } = useSpaceRewards(spacePk);
  const editingReward = useState<SpaceRewardResponse | null>(null);
  const isModalOpen = useState<boolean>(false);

  const createReward = useCreateRewardMutation();
  const updateReward = useUpdateRewardMutation();
  const deleteReward = useDeleteRewardMutation();

  return new SpaceRewardsController(
    spacePk,
    i18n,
    space,
    rewards,
    isPollSpace,
    new State(editingReward),
    new State(isModalOpen),
    createReward,
    updateReward,
    deleteReward,
  );
}
