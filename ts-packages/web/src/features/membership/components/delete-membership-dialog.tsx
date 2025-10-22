import type { MembershipResponse } from '@/features/membership/dto/list-memberships-response';
import { useMembershipsI18n } from '../i18n';

interface DeleteMembershipDialogProps {
  membership: MembershipResponse;
  onConfirm: () => Promise<void>;
  onCancel: () => void;
  isDeleting: boolean;
}

export function DeleteMembershipDialog({
  membership,
  onConfirm,
  onCancel,
  isDeleting,
}: DeleteMembershipDialogProps) {
  const i18n = useMembershipsI18n();

  return (
    <div className="flex fixed inset-0 z-50 justify-center items-center bg-black/50">
      <div className="p-6 mx-4 w-full max-w-md bg-white rounded-lg dark:bg-gray-800">
        <h2 className="mb-4 text-2xl font-bold">{i18n.deleteConfirmTitle}</h2>

        <p className="mb-6 text-gray-700 dark:text-gray-300">
          {i18n.deleteConfirmMessage.replace('{tier}', membership.tier)}
        </p>

        <div className="p-3 mb-6 bg-gray-100 rounded dark:bg-gray-700">
          <div className="space-y-1 text-sm">
            <div>
              <strong>{i18n.tier}:</strong> {membership.tier}
            </div>
            <div>
              <strong>{i18n.price}:</strong> ${membership.price_dollers}
            </div>
            <div>
              <strong>{i18n.credits}:</strong> {membership.credits}
            </div>
          </div>
        </div>

        <div className="flex gap-3">
          <button
            onClick={onConfirm}
            disabled={isDeleting}
            className="flex-1 py-2 px-4 text-white bg-red-500 rounded transition-colors hover:bg-red-600 disabled:bg-red-300"
          >
            {isDeleting ? i18n.deleting : i18n.deleteConfirm}
          </button>
          <button
            onClick={onCancel}
            disabled={isDeleting}
            className="flex-1 py-2 px-4 text-white bg-gray-500 rounded transition-colors hover:bg-gray-600 disabled:bg-gray-300"
          >
            {i18n.cancel}
          </button>
        </div>
      </div>
    </div>
  );
}
