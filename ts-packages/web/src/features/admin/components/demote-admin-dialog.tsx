import type { AdminUser } from '../types/admin-user';
import { useAdminsI18n } from '@/app/admin/users/admins-page-i18n';

interface DemoteAdminDialogProps {
  isOpen: boolean;
  admin: AdminUser | null;
  onClose: () => void;
  onConfirm: () => Promise<void>;
  isSubmitting: boolean;
  error?: string | null;
}

export function DemoteAdminDialog({
  isOpen,
  admin,
  onClose,
  onConfirm,
  isSubmitting,
  error,
}: DemoteAdminDialogProps) {
  const i18n = useAdminsI18n();

  if (!isOpen || !admin) return null;

  const handleConfirm = async () => {
    try {
      await onConfirm();
    } catch (err) {
      // Error is handled by parent
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-xl p-6 w-full max-w-md">
        <h2 className="text-2xl font-bold mb-4 text-gray-900 dark:text-gray-100">
          {i18n.demoteDialogTitle}
        </h2>
        <p className="text-gray-600 dark:text-gray-400 mb-4">
          {i18n.demoteDialogDescription}
        </p>

        <div className="mb-4 p-4 bg-gray-100 dark:bg-gray-700 rounded-md">
          <div className="flex items-center gap-3 mb-2">
            <img
              src={admin.profile_url || '/default-avatar.png'}
              alt={admin.username}
              className="w-10 h-10 rounded-full"
            />
            <div>
              <p className="font-semibold text-gray-900 dark:text-gray-100">
                {admin.display_name}
              </p>
              <p className="text-sm text-gray-600 dark:text-gray-400">
                {admin.email}
              </p>
            </div>
          </div>
        </div>

        <div className="mb-6 p-3 bg-yellow-100 dark:bg-yellow-900 text-yellow-800 dark:text-yellow-200 rounded-md text-sm">
          ⚠️ {i18n.demoteWarning}
        </div>

        {error && (
          <div className="mb-4 p-3 bg-red-100 dark:bg-red-900 text-red-700 dark:text-red-200 rounded-md text-sm">
            {error}
          </div>
        )}

        <div className="flex justify-end gap-3">
          <button
            type="button"
            onClick={onClose}
            disabled={isSubmitting}
            className="px-4 py-2 text-gray-700 dark:text-gray-300 bg-gray-200 dark:bg-gray-700 rounded-md hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors disabled:opacity-50"
          >
            {i18n.cancel}
          </button>
          <button
            type="button"
            onClick={handleConfirm}
            disabled={isSubmitting}
            className="px-4 py-2 text-white bg-red-600 rounded-md hover:bg-red-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          >
            {isSubmitting ? 'Processing...' : i18n.confirmDemote}
          </button>
        </div>
      </div>
    </div>
  );
}
