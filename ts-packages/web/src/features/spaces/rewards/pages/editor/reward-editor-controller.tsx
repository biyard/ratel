import { useState } from 'react';

import { State } from '@/types/state';
import { Space } from '@/features/spaces/types/space';
import {
  CreateSpaceRewardRequest,
  RewardAction,
  SpaceRewardResponse,
  UpdateSpaceRewardRequest,
} from '../../types';
import {
  useCreateSpaceRewardMutation,
  useUpdateSpaceRewardMutation,
  useDeleteSpaceRewardMutation,
} from '../../hooks';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { logger } from '@/lib/logger';
import { SpaceRewardsI18n, useSpaceRewardsI18n } from '../../i18n';
import { RewardUserBehavior } from '../../types/reward-user-behavior';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import usePoll from '@/features/spaces/polls/hooks/use-poll';
import { Reward } from '../../hooks/use-rewards';

export interface RewardFormData {
  behavior: RewardUserBehavior;
  credits: number;
  description?: string;
}

export interface RewardFeature {
  title: string;
  action: RewardAction;
  entityKey?: string; // Poll_SK, Board_SK, Quiz_SK, ...
}

export class RewardEditorController {
  constructor(
    public spacePk: string,
    public i18n: SpaceRewardsI18n,
    public space: Space,

    public rewardFeatures: RewardFeature[],

    public targetEntity: State<string | null>,
    public targetRewards: State<Reward[] | null>,
    public editingReward: State<SpaceRewardResponse | null>,
    public isModalOpen: State<boolean>,

    public createSpaceReward: ReturnType<typeof useCreateSpaceRewardMutation>,
    public updateSpaceReward: ReturnType<typeof useUpdateSpaceRewardMutation>,
    public deleteSpaceReward: ReturnType<typeof useDeleteSpaceRewardMutation>,
  ) {}

  openCreateModal = (entityKey: string, rewards: Reward[]) => {
    this.targetEntity.set(entityKey);
    this.targetRewards.set(rewards);
    this.isModalOpen.set(true);
  };

  openEditModal = (reward: SpaceRewardResponse) => {
    this.editingReward.set(reward);
    this.isModalOpen.set(true);
  };

  closeModal = () => {
    this.targetEntity.set(null);
    this.targetRewards.set(null);
    this.editingReward.set(null);
    this.isModalOpen.set(false);
  };

  handleCreate = async (data: RewardFormData, entity: string) => {
    try {
      const req: CreateSpaceRewardRequest = {
        action_key: entity,
        behavior: data.behavior,
        description: data.description,
        credits: data.credits,
      };

      await this.createSpaceReward.mutateAsync({
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
      const req: UpdateSpaceRewardRequest = {
        sk: reward.sk,
        description: data.description,
        credits: data.credits,
      };

      await this.updateSpaceReward.mutateAsync({
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
      await this.deleteSpaceReward.mutateAsync({
        spacePk: this.spacePk,
        req: {
          sk: reward.sk,
        },
      });

      showSuccessToast(this.i18n.settings.delete_success);
    } catch (err) {
      logger.error('Failed to delete reward', err);
      showErrorToast(this.i18n.settings.delete_error);
    }
  };

  handleSubmit = async (data: RewardFormData) => {
    const editing = this.editingReward.get();
    const entity = this.targetEntity.get();
    if (editing) {
      await this.handleUpdate(data, editing);
    } else {
      await this.handleCreate(data, entity);
    }
  };
}

export function useRewardEditorController(spacePk: string) {
  const i18n = useSpaceRewardsI18n();
  const { data: space } = useSpaceById(spacePk);

  const { data: pollsData } = usePoll(spacePk);

  // rewardFeatures
  const rewardFeatures: RewardFeature[] = [
    ...(pollsData?.polls.map((poll) => ({
      title: poll.questions[0]?.title || `Poll ${poll.sk.split('#').pop()}`,
      action: RewardAction.Poll,
      entityKey: poll.sk,
    })) || []),
  ];

  const rewards = useState<Reward[] | null>(null);
  const editingEntity = useState<string | null>(null);
  const editingReward = useState<SpaceRewardResponse | null>(null);
  const isModalOpen = useState<boolean>(false);

  const createSpaceReward = useCreateSpaceRewardMutation();
  const updateSpaceReward = useUpdateSpaceRewardMutation();
  const deleteSpaceReward = useDeleteSpaceRewardMutation();

  return new RewardEditorController(
    spacePk,
    i18n,
    space,
    rewardFeatures,
    new State(editingEntity),
    new State(rewards),
    new State(editingReward),
    new State(isModalOpen),
    createSpaceReward,
    updateSpaceReward,
    deleteSpaceReward,
  );
}
