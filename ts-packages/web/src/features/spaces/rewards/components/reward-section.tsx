import { Suspense } from 'react';
import { Col } from '@/components/ui/col';
import { Button } from '@/components/ui/button';
import { RewardCard } from './reward-card';
import { useRewards, useSpaceRewards } from '../hooks';
import { PlusIcon, ClipboardListIcon } from 'lucide-react';
import { useSpaceRewardsI18n } from '../i18n';
import { RewardAction, SpaceRewardResponse } from '../types';
import { Skeleton } from '@/components/ui/skeleton';
import { Reward } from '../hooks/use-rewards';

interface RewardSectionProps {
  title: string;
  spacePk: string;
  action: RewardAction;
  entityKey?: string;
  onAddSpaceReward: (entityKey: string, availableRewards: Reward[]) => void;
  onEditSpaceReward: (spaceReward: SpaceRewardResponse) => void;
  onDeleteSpaceReward: (spaceReward: SpaceRewardResponse) => void;
}

function RewardSectionSkeleton() {
  return (
    <div className="border border-c-wg-20 rounded-lg overflow-hidden">
      <div className="bg-c-bg-card px-4 py-3 border-b border-c-wg-20">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Skeleton className="w-4 h-4 rounded" />
            <Skeleton className="w-32 h-5" />
          </div>
        </div>
      </div>
      <div className="p-4">
        <Skeleton className="w-full h-16" />
      </div>
    </div>
  );
}

function RewardSectionContent({
  spacePk,
  action,
  entityKey,
  title,
  onAddSpaceReward,
  onEditSpaceReward,
  onDeleteSpaceReward,
}: RewardSectionProps) {
  const i18n = useSpaceRewardsI18n();
  const t = i18n.settings;

  const { data: spaceRewards = [] } = useSpaceRewards(spacePk, entityKey);
  const { data: rewards = [] } = useRewards(action);

  const hasAvailableRewardTypes = rewards.length > spaceRewards.length;

  return (
    <div className="border border-c-wg-20 rounded-lg overflow-hidden">
      <div className="bg-c-bg-card px-4 py-3 border-b border-c-wg-20">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <ClipboardListIcon className="w-4 h-4 text-c-wg-60" />
            <h3 className="font-medium text-c-wg-100">{title}</h3>
            <span className="text-xs text-c-wg-60">
              ({spaceRewards.length} {t.reward_label})
            </span>
          </div>
          {hasAvailableRewardTypes && (
            <Button
              data-testid="reward-add-button"
              variant="outline"
              size="sm"
              onClick={() => onAddSpaceReward(entityKey, rewards)}
            >
              <PlusIcon className="w-4 h-4" />
              {t.create_reward}
            </Button>
          )}
        </div>
      </div>

      <div className="p-4">
        {spaceRewards.length === 0 ? (
          <div className="text-center py-4 text-c-wg-60 text-sm">
            {t.no_rewards}
          </div>
        ) : (
          <Col className="gap-3">
            {spaceRewards.map((reward) => (
              <RewardCard
                key={reward.sk}
                i18n={i18n}
                reward={reward}
                onEdit={() => onEditSpaceReward(reward)}
                onDelete={() => onDeleteSpaceReward(reward)}
              />
            ))}
          </Col>
        )}
      </div>
    </div>
  );
}

export function RewardSection(props: RewardSectionProps) {
  return (
    <Suspense fallback={<RewardSectionSkeleton />}>
      <RewardSectionContent {...props} />
    </Suspense>
  );
}
