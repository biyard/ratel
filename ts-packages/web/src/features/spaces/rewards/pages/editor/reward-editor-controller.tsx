import { useState } from 'react';

import { State } from '@/types/state';
import { Space } from '@/features/spaces/types/space';
import { SpaceRewardResponse } from '../../types/space-reward-response';
import { useCreateRewardMutation } from '../../hooks/use-create-reward-mutation';
import { useUpdateRewardMutation } from '../../hooks/use-update-reward-mutation';
import { useDeleteRewardMutation } from '../../hooks/use-delete-reward-mutation';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { logger } from '@/lib/logger';
import { CreateRewardRequest } from '../../types/create-reward-request';
import { UpdateRewardRequest } from '../../types/update-reward-request';
import { SpaceRewardsI18n, useSpaceRewardsI18n } from '../../i18n';
import {
  convertRewardActionToRequest,
  RewardAction,
} from '../../types/reward-type';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import usePoll from '@/features/spaces/polls/hooks/use-poll';
import { FeatureType } from '../../types/feature-type';
import { RewardConfigItem } from '../../hooks/use-reward-config';

export interface RewardFormData {
  reward_action: RewardAction;
  credits: number;
  description?: string;
}

export interface RewardFeature {
  title: string;
  entityType: string;
  featureType: FeatureType;
}

export class RewardEditorController {
  constructor(
    public spacePk: string,
    public i18n: SpaceRewardsI18n,
    public space: Space,

    public rewardFeatures: RewardFeature[],

    public targetEntity: State<string | null>,
    public targetRewardConfigs: State<RewardConfigItem[] | null>,
    public editingReward: State<SpaceRewardResponse | null>,
    public isModalOpen: State<boolean>,

    public createReward: ReturnType<typeof useCreateRewardMutation>,
    public updateReward: ReturnType<typeof useUpdateRewardMutation>,
    public deleteReward: ReturnType<typeof useDeleteRewardMutation>,
  ) {}

  openCreateModal = (entity: string, configs: RewardConfigItem[]) => {
    this.targetEntity.set(entity);
    this.targetRewardConfigs.set(configs);
    // this.editingReward.set(null);
    this.isModalOpen.set(true);
  };

  openEditModal = (reward: SpaceRewardResponse) => {
    // this.targetEntity.set(null);
    // this.targetRewardConfigs.set(null);
    this.editingReward.set(reward);
    this.isModalOpen.set(true);
  };

  closeModal = () => {
    this.targetEntity.set(null);
    this.targetRewardConfigs.set(null);
    this.editingReward.set(null);
    this.isModalOpen.set(false);
  };

  handleCreate = async (data: RewardFormData, entity: string) => {
    try {
      console.log('REWARD_ACTION', data.reward_action);
      const req: CreateRewardRequest = {
        reward: convertRewardActionToRequest(data.reward_action, entity),
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
      entityType: poll.sk,
      featureType: FeatureType.POLL,
    })) || []),
  ];

  const configs = useState<RewardConfigItem[] | null>(null);
  const editingEntity = useState<string | null>(null);
  const editingReward = useState<SpaceRewardResponse | null>(null);
  const isModalOpen = useState<boolean>(false);

  const createReward = useCreateRewardMutation();
  const updateReward = useUpdateRewardMutation();
  const deleteReward = useDeleteRewardMutation();

  return new RewardEditorController(
    spacePk,
    i18n,
    space,
    rewardFeatures,
    new State(editingEntity),
    new State(configs),
    new State(editingReward),
    new State(isModalOpen),
    createReward,
    updateReward,
    deleteReward,
  );
}
