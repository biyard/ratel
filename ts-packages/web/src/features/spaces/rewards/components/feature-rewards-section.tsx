import { Col } from '@/components/ui/col';
import { Button } from '@/components/ui/button';
import { Poll } from '@/features/spaces/polls/types/poll';
import { SpaceRewardResponse } from '../types';
import { RewardCard } from './reward-card';
import { PlusIcon } from 'lucide-react';
import { SpaceRewardsI18n } from '../i18n';

interface FeatureRewardsSectionProps {
  i18n: SpaceRewardsI18n;
  poll: Poll;
  reward: SpaceRewardResponse | null;
  onCreateReward: (pollSk: string) => void;
  onEditReward: (reward: SpaceRewardResponse, pollSk: string) => void;
  onDeleteReward: (pollSk: string) => void;
}

export function FeatureRewardsSection({
  i18n,
  poll,
  reward,
  onCreateReward,
  onEditReward,
  onDeleteReward,
}: FeatureRewardsSectionProps) {
  const t = i18n.settings;
  const pollTitle =
    poll.questions.length > 0
      ? poll.questions[0].title || `Poll #${poll.sk.slice(-6)}`
      : `Poll #${poll.sk.slice(-6)}`;
  return (
    <div className="border border-c-wg-20 rounded-lg p-4 bg-c-bg-card">
      <Col className="gap-4">
        <div className="flex justify-between items-center">
          <div>
            <h3 className="text-base font-semibold text-c-wg-100">
              {pollTitle}
            </h3>
            <p className="text-sm text-c-wg-60">{t.poll_reward_section}</p>
          </div>
          <Button
            variant="outline"
            size="sm"
            onClick={() => onCreateReward(poll.sk)}
          >
            <PlusIcon className="w-4 h-4" />
            {t.create_reward}
          </Button>
        </div>

        {reward ? (
          <RewardCard
            i18n={i18n}
            reward={reward}
            onEdit={() => onEditReward(reward, poll.sk)}
            onDelete={() => onDeleteReward(poll.sk)}
          />
        ) : (
          <div className="text-center py-4 text-c-wg-60 text-sm">
            {t.no_rewards}
            <br />
            <span className="text-xs">{t.no_rewards_description}</span>
          </div>
        )}
      </Col>
    </div>
  );
}
