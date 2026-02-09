import type {
  AdminPaymentResponse,
  AdminCancelPaymentResponse,
} from '@/features/admin/types/admin-user';
import type { AdminPaymentsI18n } from '@/app/admin/payments/payments-page-i18n';
import { formatCurrency, formatDate } from '@/features/admin/utils/format';
import { Button } from '@/components/ui/button';

interface RefundModalProps {
  isOpen: boolean;
  onClose: () => void;
  payment: AdminPaymentResponse | null;
  i18n: AdminPaymentsI18n;
  // State props
  reason: string;
  error: string | null;
  success: AdminCancelPaymentResponse | null;
  isProcessing: boolean;
  // Handler props
  onReasonChange: (reason: string) => void;
  onSubmit: () => void;
  onSuccessConfirm: () => void;
}

export function RefundModal({
  isOpen,
  onClose,
  payment,
  i18n,
  reason,
  error,
  success,
  isProcessing,
  onReasonChange,
  onSubmit,
  onSuccessConfirm,
}: RefundModalProps) {
  if (!isOpen || !payment) return null;

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSubmit();
  };

  if (success) {
    return (
      <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
        <div className="w-full max-w-md rounded-lg bg-white p-6 shadow-xl dark:bg-gray-800">
          <div className="mb-4 flex items-center justify-center">
            <div className="flex h-12 w-12 items-center justify-center rounded-full bg-green-100 dark:bg-green-900">
              <svg
                className="h-6 w-6 text-green-600 dark:text-green-300"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M5 13l4 4L19 7"
                />
              </svg>
            </div>
          </div>
          <h2 className="mb-4 text-center text-xl font-semibold text-gray-900 dark:text-white">
            {i18n.refundSuccessTitle}
          </h2>
          <div className="mb-6 space-y-2 rounded-lg bg-gray-50 p-4 dark:bg-gray-700">
            <div className="flex justify-between text-sm">
              <span className="text-gray-600 dark:text-gray-400">
                {i18n.refundCancellationId}
              </span>
              <span className="font-medium text-gray-900 dark:text-gray-100">
                {success.cancellation_id}
              </span>
            </div>
            <div className="flex justify-between text-sm">
              <span className="text-gray-600 dark:text-gray-400">
                {i18n.refundAmountLabel}
              </span>
              <span className="font-medium text-gray-900 dark:text-gray-100">
                {formatCurrency(success.total_amount, payment.currency)}
              </span>
            </div>
            <div className="flex justify-between text-sm">
              <span className="text-gray-600 dark:text-gray-400">
                {i18n.refundReasonLabel}
              </span>
              <span className="font-medium text-gray-900 dark:text-gray-100">
                {success.reason}
              </span>
            </div>
            {success.cancelled_at && (
              <div className="flex justify-between text-sm">
                <span className="text-gray-600 dark:text-gray-400">
                  {i18n.refundCancelledAt}
                </span>
                <span className="font-medium text-gray-900 dark:text-gray-100">
                  {formatDate(success.cancelled_at)}
                </span>
              </div>
            )}
          </div>
          <Button
            onClick={onSuccessConfirm}
            className="w-full"
            variant="rounded_primary"
          >
            {i18n.refundSuccessConfirm}
          </Button>
        </div>
      </div>
    );
  }

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div className="w-full max-w-md rounded-lg bg-white p-6 shadow-xl dark:bg-gray-800">
        <h2 className="mb-4 text-xl font-semibold text-gray-900 dark:text-white">
          {i18n.refundTitle}
        </h2>

        {error && (
          <div className="mb-4 rounded-lg bg-red-50 p-3 dark:bg-red-900/20">
            <p className="text-sm text-red-600 dark:text-red-400">{error}</p>
          </div>
        )}

        <form onSubmit={handleSubmit}>
          <div className="mb-4">
            <label className="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
              {i18n.paymentId}
            </label>
            <input
              type="text"
              value={payment.payment_id}
              disabled
              className="w-full rounded-md border border-gray-300 bg-gray-100 px-3 py-2 text-sm dark:border-gray-600 dark:bg-gray-700"
            />
          </div>
          <div className="mb-4">
            <label className="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
              {i18n.refundReason}
            </label>
            <textarea
              value={reason}
              onChange={(e) => onReasonChange(e.target.value)}
              placeholder={i18n.refundReasonPlaceholder}
              className="w-full rounded-md border border-gray-300 px-3 py-2 text-sm dark:border-gray-600 dark:bg-gray-700"
              rows={3}
              required
              disabled={isProcessing}
            />
          </div>
          <div className="flex justify-end gap-3">
            <Button
              type="button"
              onClick={onClose}
              disabled={isProcessing}
              variant="rounded_secondary"
            >
              {i18n.refundCancel}
            </Button>
            <Button
              type="submit"
              disabled={isProcessing}
              variant="rounded_primary"
              className="bg-red-600 hover:bg-red-700 text-white border-red-600 hover:border-red-700"
            >
              {isProcessing ? i18n.refundProcessing : i18n.refundSubmit}
            </Button>
          </div>
        </form>
      </div>
    </div>
  );
}
