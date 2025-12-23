import { useState } from 'react';
import { Col } from '@/components/ui/col';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { SpaceRewardResponse } from '../types/space-reward-response';
import { RewardFormData } from '@/app/spaces/[id]/rewards/use-space-rewards-controller';
import { RewardsI18n } from '../i18n';
import { RewardConfig } from '../types/reward-config';
import { Poll } from '@/features/spaces/polls/types/poll';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';

interface RewardFormProps {
  i18n: RewardsI18n;
  initialData?: SpaceRewardResponse | null;
  availableRewards: RewardConfig[];
  polls: Poll[];
  onSubmit: (data: RewardFormData) => Promise<void>;
  onCancel: () => void;
  isSubmitting: boolean;
}

export function RewardForm({
  i18n,
  initialData,
  availableRewards,
  polls,
  onSubmit,
  onCancel,
  isSubmitting,
}: RewardFormProps) {
  const t = i18n.settings;
  const isEditing = !!initialData;

  const [pollSk, setPollSk] = useState<string>(
    initialData?.getPollSk() ?? polls[0]?.sk ?? '',
  );
  const [description, setDescription] = useState(
    initialData?.description ?? '',
  );
  const [credits, setCredits] = useState(initialData?.credits ?? 1);
  const [errors, setErrors] = useState<Record<string, string>>({});

  const selectedConfig = availableRewards[0];

  const validate = (): boolean => {
    const newErrors: Record<string, string> = {};

    if (!isEditing && !pollSk) {
      newErrors.pollSk = 'Poll is required';
    }

    if (credits < 1) {
      newErrors.credits = t.credits_min;
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!validate()) return;

    await onSubmit({
      pollSk,
      description: description.trim(),
      credits,
    });
  };

  const getPollTitle = (poll: Poll): string => {
    return poll.questions.length > 0
      ? poll.questions[0].title || `Poll #${poll.sk.slice(-6)}`
      : `Poll #${poll.sk.slice(-6)}`;
  };

  return (
    <form onSubmit={handleSubmit}>
      <Col className="gap-4">
        {!isEditing && polls.length > 0 && (
          <div>
            <label className="block text-sm font-medium text-c-wg-80 mb-1">
              {t.poll_reward_section}
            </label>
            <Select value={pollSk} onValueChange={(value) => setPollSk(value)}>
              <SelectTrigger>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {polls.map((poll) => (
                  <SelectItem key={poll.sk} value={poll.sk}>
                    {getPollTitle(poll)}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            {errors.pollSk && (
              <p className="text-sm text-red-500 mt-1">{errors.pollSk}</p>
            )}
          </div>
        )}

        {selectedConfig && (
          <div>
            <label className="block text-sm font-medium text-c-wg-80 mb-1">
              {t.points}
            </label>
            <Input disabled value={selectedConfig.point.toLocaleString()} />
          </div>
        )}

        <div>
          <label className="block text-sm font-medium text-c-wg-80 mb-1">
            {t.description}
          </label>
          <Input
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            placeholder={t.description_placeholder}
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-c-wg-80 mb-1">
            {t.credits}
          </label>
          <Input
            type="number"
            value={credits}
            onChange={(e) => setCredits(parseInt(e.target.value) || 0)}
            placeholder={t.credits_placeholder}
            min={1}
            aria-invalid={!!errors.credits}
          />
          {errors.credits && (
            <p className="text-sm text-red-500 mt-1">{errors.credits}</p>
          )}
        </div>

        <div className="flex gap-3 justify-end mt-4">
          <Button
            type="button"
            variant="outline"
            onClick={onCancel}
            disabled={isSubmitting}
          >
            {t.cancel}
          </Button>
          <Button type="submit" variant="primary" disabled={isSubmitting}>
            {isSubmitting ? t.loading : t.save}
          </Button>
        </div>
      </Col>
    </form>
  );
}
