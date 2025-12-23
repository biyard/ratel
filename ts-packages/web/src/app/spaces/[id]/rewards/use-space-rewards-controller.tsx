import { useState } from 'react';

import { State } from '@/types/state';
import { Space } from '@/features/spaces/types/space';
import { SpaceRewardResponse } from '@/features/spaces/rewards/types/space-reward-response';
import { ListRewardsResponse } from '@/features/spaces/rewards/types/list-rewards-response';
import { ListPollResponse } from '@/features/spaces/polls/types/list-poll-response';
import { Poll } from '@/features/spaces/polls/types/poll';
import { useCreateRewardMutation } from '@/features/spaces/rewards/hooks/use-create-reward-mutation';
import { useUpdateRewardMutation } from '@/features/spaces/rewards/hooks/use-update-reward-mutation';
import { useDeleteRewardMutation } from '@/features/spaces/rewards/hooks/use-delete-reward-mutation';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { logger } from '@/lib/logger';
import { CreateRewardRequest } from '@/features/spaces/rewards/types/create-reward-request';
import { UpdateRewardRequest } from '@/features/spaces/rewards/types/update-reward-request';
import { useSpaceRewardsData } from './use-space-rewards-data';
import { RewardsI18n, useRewardsI18n } from '@/features/spaces/rewards/i18n';

export interface RewardFormData {
  description?: string;
  credits: number;
}

export interface RewardsByPoll {
  poll: Poll;
  rewards: SpaceRewardResponse[];
}

export class SpaceRewardsController {
  constructor(
    public spacePk: string,
    public i18n: RewardsI18n,
    public space: Space,
    public rewards: ListRewardsResponse,
    public polls: ListPollResponse | null,
    public isPollSpace: boolean,

    public selectedPollSk: State<string | null>,
    public editingReward: State<SpaceRewardResponse | null>,
    public isModalOpen: State<boolean>,

    public createReward: ReturnType<typeof useCreateRewardMutation>,
    public updateReward: ReturnType<typeof useUpdateRewardMutation>,
    public deleteReward: ReturnType<typeof useDeleteRewardMutation>,
  ) {}

  getRewardsByPoll(): RewardsByPoll[] {
    if (!this.polls || !this.isPollSpace) {
      return [];
    }

    return this.polls.polls.map((poll) => {
      const pollRewards = this.rewards.items.filter((reward) => {
        // sk format: "POLL#<poll_sk>#Respond"
        return reward.sk.startsWith(`POLL#${poll.sk}#`);
      });

      return {
        poll,
        rewards: pollRewards,
      };
    });
  }

  getRewardForPoll(pollSk: string): SpaceRewardResponse | null {
    return (
      this.rewards.items.find((reward) =>
        reward.sk.startsWith(`POLL#${pollSk}#`),
      ) ?? null
    );
  }

  openCreateModal = (pollSk: string) => {
    this.selectedPollSk.set(pollSk);
    this.editingReward.set(null);
    this.isModalOpen.set(true);
  };

  openEditModal = (reward: SpaceRewardResponse, pollSk: string) => {
    this.selectedPollSk.set(pollSk);
    this.editingReward.set(reward);
    this.isModalOpen.set(true);
  };

  closeModal = () => {
    this.selectedPollSk.set(null);
    this.editingReward.set(null);
    this.isModalOpen.set(false);
  };

  handleCreate = async (data: RewardFormData) => {
    const pollSk = this.selectedPollSk.get();
    if (!pollSk) return;

    try {
      const req: CreateRewardRequest = {
        reward: { poll_sk: pollSk },
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

  handleUpdate = async (data: RewardFormData) => {
    const pollSk = this.selectedPollSk.get();
    if (!pollSk) return;

    try {
      const req: UpdateRewardRequest = {
        reward: { poll_sk: pollSk },
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

  handleDelete = async (pollSk: string) => {
    try {
      await this.deleteReward.mutateAsync({
        spacePk: this.spacePk,
        req: { reward: { poll_sk: pollSk } },
      });

      showSuccessToast(this.i18n.settings.delete_success);
    } catch (err) {
      logger.error('Failed to delete reward', err);
      showErrorToast(this.i18n.settings.delete_error);
    }
  };

  handleSubmit = async (data: RewardFormData) => {
    if (this.editingReward.get()) {
      await this.handleUpdate(data);
    } else {
      await this.handleCreate(data);
    }
  };
}

export function useSpaceRewardsController(spacePk: string) {
  const i18n = useRewardsI18n();
  const { space, rewards, polls, isPollSpace } = useSpaceRewardsData(spacePk);

  const selectedPollSk = useState<string | null>(null);
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
    polls,
    isPollSpace,
    new State(selectedPollSk),
    new State(editingReward),
    new State(isModalOpen),
    createReward,
    updateReward,
    deleteReward,
  );
}
