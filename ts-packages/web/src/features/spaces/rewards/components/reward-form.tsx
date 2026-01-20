import { useState } from 'react';
import { Col } from '@/components/ui/col';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { SpaceRewardResponse } from '../types/space-reward-response';
import { RewardsI18n } from '../i18n';
import { RewardAction, getRewardActionI18nKey } from '../types/reward-type';
import { RewardFormData } from '../pages/editor/reward-editor-controller';

interface RewardFormProps {
  i18n: RewardsI18n;
  initialData?: SpaceRewardResponse | null;
  onSubmit: (data: RewardFormData) => Promise<void>;
  onCancel: () => void;
  isSubmitting: boolean;
  rewardActions: RewardAction[];
}

export function RewardForm({
  i18n,
  initialData,
  onSubmit,
  onCancel,
  isSubmitting,
  rewardActions,
}: RewardFormProps) {
  const t = i18n.settings;

  const [action, setAction] = useState<RewardAction | undefined>(
    initialData?.reward_action ?? rewardActions[0] ?? undefined,
  );
  const [description, setDescription] = useState(
    initialData?.description ?? '',
  );
  const [credits, setCredits] = useState(initialData?.credits ?? 1);
  const [errors, setErrors] = useState<Record<string, string>>({});

  const validate = (): boolean => {
    const newErrors: Record<string, string> = {};

    if (!action) {
      newErrors.action = t.reward_type_required;
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
      reward_action: action!,
      description: description.trim(),
      credits,
    });
  };

  return (
    <form onSubmit={handleSubmit}>
      <Col className="gap-4">
        <div>
          <label className="block text-sm font-medium text-c-wg-80 mb-1">
            {t.reward_action}
          </label>
          <Select
            value={action}
            onValueChange={(value) => setAction(value as RewardAction)}
          >
            <SelectTrigger
              data-testid="reward-action-select"
              className="w-full"
              aria-invalid={!!errors.action}
            >
              <SelectValue placeholder={t.select_reward_type} />
            </SelectTrigger>
            <SelectContent>
              {rewardActions.map((rewardAction) => (
                <SelectItem key={rewardAction} value={rewardAction}>
                  {t[getRewardActionI18nKey(rewardAction) as keyof typeof t] ||
                    rewardAction}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
          {errors.action && (
            <p className="text-sm text-red-500 mt-1">{errors.action}</p>
          )}
        </div>

        <div>
          <label className="block text-sm font-medium text-c-wg-80 mb-1">
            {t.credits}
          </label>
          <Input
            data-testid="reward-credits-input"
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

        <div>
          <label className="block text-sm font-medium text-c-wg-80 mb-1">
            {t.description}
          </label>
          <Input
            data-testid="reward-description-input"
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            placeholder={t.description_placeholder}
          />
        </div>

        <div className="flex gap-3 justify-end mt-4">
          <Button
            data-testid="reward-cancel-button"
            type="button"
            variant="outline"
            onClick={onCancel}
            disabled={isSubmitting}
          >
            {t.cancel}
          </Button>
          <Button
            data-testid="reward-save-button"
            type="submit"
            variant="primary"
            disabled={isSubmitting}
          >
            {isSubmitting ? t.loading : t.save}
          </Button>
        </div>
      </Col>
    </form>
  );
}
