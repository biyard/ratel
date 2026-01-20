import { Suspense } from 'react';
import { Col } from '@/components/ui/col';
import { Button } from '@/components/ui/button';
import { RewardCard } from './reward-card';
import useSpaceRewards from '../hooks/use-space-rewards';
import useRewardConfig, { RewardConfigItem } from '../hooks/use-reward-config';
import { PlusIcon, ClipboardListIcon } from 'lucide-react';
import { useRewardsI18n } from '../i18n';
import { SpaceRewardResponse } from '../types/space-reward-response';
import { FeatureType } from '../types/feature-type';
import { Skeleton } from '@/components/ui/skeleton';

interface RewardSectionProps {
  spacePk: string;
  featureType: FeatureType;
  entityType: string;
  entityTitle: string;
  onAddReward: (entityType: string, configs: RewardConfigItem[]) => void;
  onEditReward: (reward: SpaceRewardResponse, entityType: string) => void;
  onDeleteReward: (reward: SpaceRewardResponse) => void;
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
  featureType,
  entityTitle,
  entityType,
  onAddReward,
  onEditReward,
  onDeleteReward,
}: RewardSectionProps) {
  const i18n = useRewardsI18n();
  const t = i18n.settings;

  const {
    data: { items: rewards },
  } = useSpaceRewards(spacePk, entityType);
  const { data: rewardConfig } = useRewardConfig(featureType);

  const hasAvailableRewardTypes = rewardConfig.items.length >= rewards.length;

  return (
    <div className="border border-c-wg-20 rounded-lg overflow-hidden">
      <div className="bg-c-bg-card px-4 py-3 border-b border-c-wg-20">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <ClipboardListIcon className="w-4 h-4 text-c-wg-60" />
            <h3 className="font-medium text-c-wg-100">{entityTitle}</h3>
            <span className="text-xs text-c-wg-60">
              ({rewards.length} {t.reward_label})
            </span>
          </div>
          {hasAvailableRewardTypes && (
            <Button
              data-testid="reward-add-button"
              variant="outline"
              size="sm"
              onClick={() => onAddReward(entityType, rewardConfig.items)}
            >
              <PlusIcon className="w-4 h-4" />
              {t.create_reward}
            </Button>
          )}
        </div>
      </div>

      <div className="p-4">
        {rewards.length === 0 ? (
          <div className="text-center py-4 text-c-wg-60 text-sm">
            {t.no_rewards}
          </div>
        ) : (
          <Col className="gap-3">
            {rewards.map((reward) => (
              <RewardCard
                key={reward.sk}
                i18n={i18n}
                reward={reward}
                onEdit={() => onEditReward(reward, entityType)}
                onDelete={() => onDeleteReward(reward)}
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
