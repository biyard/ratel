import { useState, useEffect } from 'react';
import { useMembershipsI18n } from '../i18n';
import { MembershipResponse } from '../dto/membership-response';
import { CreateMembershipRequest } from '../dto/create-membership-request';
import { UpdateMembershipRequest } from '../dto/update-membership-request';
import { MembershipTier } from '../types/membership-tier';

interface MembershipFormProps {
  membership: MembershipResponse | null;
  onSubmit: (
    data: CreateMembershipRequest | UpdateMembershipRequest,
  ) => Promise<void>;
  onCancel: () => void;
  isSubmitting: boolean;
}

export function MembershipForm({
  membership,
  onSubmit,
  onCancel,
  isSubmitting,
}: MembershipFormProps) {
  const i18n = useMembershipsI18n();
  const isEditing = !!membership;

  const [formData, setFormData] = useState<CreateMembershipRequest>({
    tier: MembershipTier.Free,
    price_dollars: 0,
    credits: 0,
    duration_days: 30,
    display_order: 0,
  });

  const [isActive, setIsActive] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (membership) {
      setFormData({
        tier: membership.tier,
        price_dollars: membership.price_dollars,
        credits: membership.credits,
        duration_days: membership.duration_days,
        display_order: membership.display_order,
      });
      setIsActive(membership.is_active);
    }
  }, [membership]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);

    try {
      if (isEditing) {
        const updateData: UpdateMembershipRequest = {
          ...formData,
          is_active: isActive,
        };
        await onSubmit(updateData);
      } else {
        await onSubmit(formData);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : i18n.submitError);
    }
  };

  const handleChange = (
    e: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>,
  ) => {
    const { name, value } = e.target;
    setFormData((prev) => ({
      ...prev,
      [name]:
        name === 'tier'
          ? value
          : name === 'price_dollars' ||
              name === 'credits' ||
              name === 'duration_days' ||
              name === 'display_order'
            ? parseInt(value) || 0
            : value,
    }));
  };

  return (
    <div className="flex fixed inset-0 z-50 justify-center items-center bg-black/50">
      <div className="overflow-y-auto p-6 mx-4 w-full max-w-md bg-white rounded-lg dark:bg-gray-800 max-h-[90vh]">
        <h2 className="mb-4 text-2xl font-bold">
          {isEditing ? i18n.editMembership : i18n.createMembership}
        </h2>

        {error && (
          <div className="p-3 mb-4 text-red-800 bg-red-100 rounded dark:text-red-200 dark:bg-red-900">
            {error}
          </div>
        )}

        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="block mb-1 text-sm font-medium">
              {i18n.tier}
            </label>
            <select
              name="tier"
              value={formData.tier}
              onChange={handleChange}
              className="p-2 w-full bg-white rounded border border-gray-300 dark:bg-gray-700 dark:border-gray-600"
              required
            >
              <option value={MembershipTier.Free}>{MembershipTier.Free}</option>
              <option value={MembershipTier.Pro}>{MembershipTier.Pro}</option>
              <option value={MembershipTier.Max}>{MembershipTier.Max}</option>
              <option value={MembershipTier.Vip}>{MembershipTier.Vip}</option>
            </select>
          </div>

          <div>
            <label className="block mb-1 text-sm font-medium">
              {i18n.price} ($)
            </label>
            <input
              type="number"
              name="price_dollars"
              value={formData.price_dollars}
              onChange={handleChange}
              className="p-2 w-full bg-white rounded border border-gray-300 dark:bg-gray-700 dark:border-gray-600"
              min="0"
              required
            />
          </div>

          <div>
            <label className="block mb-1 text-sm font-medium">
              {i18n.credits}
            </label>
            <input
              type="number"
              name="credits"
              value={formData.credits}
              onChange={handleChange}
              className="p-2 w-full bg-white rounded border border-gray-300 dark:bg-gray-700 dark:border-gray-600"
              min="0"
              required
            />
          </div>

          <div>
            <label className="block mb-1 text-sm font-medium">
              {i18n.duration} ({i18n.days})
            </label>
            <input
              type="number"
              name="duration_days"
              value={formData.duration_days}
              onChange={handleChange}
              className="p-2 w-full bg-white rounded border border-gray-300 dark:bg-gray-700 dark:border-gray-600"
              min="1"
              required
            />
          </div>

          <div>
            <label className="block mb-1 text-sm font-medium">
              {i18n.displayOrder}
            </label>
            <input
              type="number"
              name="display_order"
              value={formData.display_order}
              onChange={handleChange}
              className="p-2 w-full bg-white rounded border border-gray-300 dark:bg-gray-700 dark:border-gray-600"
              min="0"
              required
            />
          </div>

          {isEditing && (
            <div className="flex items-center">
              <input
                type="checkbox"
                id="is_active"
                checked={isActive}
                onChange={(e) => setIsActive(e.target.checked)}
                className="mr-2"
              />
              <label htmlFor="is_active" className="text-sm font-medium">
                {i18n.isActive}
              </label>
            </div>
          )}

          <div className="flex gap-3 pt-4">
            <button
              type="submit"
              disabled={isSubmitting}
              className="flex-1 py-2 px-4 text-white bg-blue-500 rounded transition-colors hover:bg-blue-600 disabled:bg-blue-300"
            >
              {isSubmitting ? i18n.submitting : i18n.submit}
            </button>
            <button
              type="button"
              onClick={onCancel}
              disabled={isSubmitting}
              className="flex-1 py-2 px-4 text-white bg-gray-500 rounded transition-colors hover:bg-gray-600 disabled:bg-gray-300"
            >
              {i18n.cancel}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
