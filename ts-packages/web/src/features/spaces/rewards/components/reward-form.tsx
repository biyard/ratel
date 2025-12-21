import { useState } from 'react';
import { Col } from '@/components/ui/col';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { SpaceRewardResponse } from '../types/space-reward-response';
import { RewardFormData } from '@/app/spaces/[id]/rewards/use-space-rewards-controller';
import { RewardsI18n } from '../i18n';

interface RewardFormProps {
  i18n: RewardsI18n;
  initialData?: SpaceRewardResponse | null;
  onSubmit: (data: RewardFormData) => Promise<void>;
  onCancel: () => void;
  isSubmitting: boolean;
}

export function RewardForm({
  i18n,
  initialData,
  onSubmit,
  onCancel,
  isSubmitting,
}: RewardFormProps) {
  const t = i18n.settings;
  const [label, setLabel] = useState(initialData?.label ?? '');
  const [description, setDescription] = useState(
    initialData?.description ?? '',
  );
  const [credits, setCredits] = useState(initialData?.credits ?? 10);
  const [errors, setErrors] = useState<Record<string, string>>({});

  const validate = (): boolean => {
    const newErrors: Record<string, string> = {};

    if (!label.trim()) {
      newErrors.label = t.label_required;
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
      label: label.trim(),
      description: description.trim(),
      credits,
    });
  };

  return (
    <form onSubmit={handleSubmit}>
      <Col className="gap-4">
        <div>
          <label className="block text-sm font-medium text-c-wg-80 mb-1">
            {t.reward_label}
          </label>
          <Input
            value={label}
            onChange={(e) => setLabel(e.target.value)}
            placeholder={t.label_placeholder}
            aria-invalid={!!errors.label}
          />
          {errors.label && (
            <p className="text-sm text-red-500 mt-1">{errors.label}</p>
          )}
        </div>

        <div>
          <label className="block text-sm font-medium text-c-wg-80 mb-1">
            Description
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
