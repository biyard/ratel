import { useState } from 'react';
import { useAdminsI18n } from '@/app/admin/users/admins-page-i18n';

interface PromoteAdminDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onSubmit: (email: string) => Promise<void>;
  isSubmitting: boolean;
  error?: string | null;
}

export function PromoteAdminDialog({
  isOpen,
  onClose,
  onSubmit,
  isSubmitting,
  error,
}: PromoteAdminDialogProps) {
  const i18n = useAdminsI18n();
  const [email, setEmail] = useState('');

  if (!isOpen) return null;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!email.trim()) return;

    try {
      await onSubmit(email.trim());
      setEmail('');
    } catch (err) {
      // Error is handled by parent
    }
  };

  const handleClose = () => {
    setEmail('');
    onClose();
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow-xl p-6 w-full max-w-md">
        <h2 className="text-2xl font-bold mb-4 text-gray-900 dark:text-gray-100">
          {i18n.promoteDialogTitle}
        </h2>
        <p className="text-gray-600 dark:text-gray-400 mb-6">
          {i18n.promoteDialogDescription}
        </p>

        <form onSubmit={handleSubmit}>
          <div className="mb-4">
            <label
              htmlFor="email"
              className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2"
            >
              {i18n.email}
            </label>
            <input
              id="email"
              type="email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              placeholder={i18n.emailPlaceholder}
              className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-gray-100"
              required
              disabled={isSubmitting}
            />
          </div>

          {error && (
            <div className="mb-4 p-3 bg-red-100 dark:bg-red-900 text-red-700 dark:text-red-200 rounded-md text-sm">
              {error}
            </div>
          )}

          <div className="flex justify-end gap-3">
            <button
              type="button"
              onClick={handleClose}
              disabled={isSubmitting}
              className="px-4 py-2 text-gray-700 dark:text-gray-300 bg-gray-200 dark:bg-gray-700 rounded-md hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors disabled:opacity-50"
            >
              {i18n.cancel}
            </button>
            <button
              type="submit"
              disabled={isSubmitting || !email.trim()}
              className="px-4 py-2 text-white bg-blue-600 rounded-md hover:bg-blue-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
            >
              {isSubmitting ? 'Processing...' : i18n.promoteAdmin}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
