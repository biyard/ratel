import { useState, useEffect } from 'react';
import type {
  AdminPaymentResponse,
  AdminCancelPaymentResponse,
} from '@/features/admin/types/admin-user';
import { RefundRequester } from '@/features/admin/types/admin-user';
import { useCancelPaymentMutation } from '@/features/admin/hooks/use-cancel-payment-mutation';
import type { AdminPaymentsI18n } from '@/app/admin/payments/payments-page-i18n';

function formatCurrency(amount: number, currency: string): string {
  return new Intl.NumberFormat('ko-KR', {
    style: 'currency',
    currency: currency || 'KRW',
  }).format(amount);
}

function formatDate(dateStr: string | null): string {
  if (!dateStr) return '-';
  const date = new Date(dateStr);
  return new Intl.DateTimeFormat('ko-KR', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
  }).format(date);
}

interface RefundModalProps {
  isOpen: boolean;
  onClose: () => void;
  payment: AdminPaymentResponse | null;
  i18n: AdminPaymentsI18n;
}

export function RefundModal({
  isOpen,
  onClose,
  payment,
  i18n,
}: RefundModalProps) {
  const [reason, setReason] = useState('');
  const [amount, setAmount] = useState(0);
  const [requester, setRequester] = useState<RefundRequester>(
    RefundRequester.Admin,
  );
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<AdminCancelPaymentResponse | null>(
    null,
  );
  const cancelMutation = useCancelPaymentMutation();

  useEffect(() => {
    if (payment) {
      const refundableAmount = payment.total - (payment.cancelled || 0);
      setAmount(refundableAmount);
      setError(null);
      setSuccess(null);
    }
  }, [payment]);

  if (!isOpen || !payment) return null;

  const maxAmount = payment.total - (payment.cancelled || 0);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!payment.user_pk) {
      setError('사용자 정보를 찾을 수 없습니다.');
      return;
    }

    setError(null);

    try {
      const response = await cancelMutation.mutateAsync({
        paymentId: payment.payment_id,
        request: {
          reason,
          amount: amount === maxAmount ? undefined : amount,
          requester,
          user_pk: payment.user_pk,
        },
      });

      setSuccess(response);
    } catch (err) {
      setError(
        err instanceof Error
          ? err.message
          : '환불 처리 중 오류가 발생했습니다.',
      );
    }
  };

  const handleClose = () => {
    setReason('');
    setError(null);
    setSuccess(null);
    onClose();
  };

  const handleSuccessConfirm = () => {
    handleClose();
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
            환불이 완료되었습니다
          </h2>
          <div className="mb-6 space-y-2 rounded-lg bg-gray-50 p-4 dark:bg-gray-700">
            <div className="flex justify-between text-sm">
              <span className="text-gray-600 dark:text-gray-400">취소 ID</span>
              <span className="font-medium text-gray-900 dark:text-gray-100">
                {success.cancellation_id}
              </span>
            </div>
            <div className="flex justify-between text-sm">
              <span className="text-gray-600 dark:text-gray-400">
                환불 금액
              </span>
              <span className="font-medium text-gray-900 dark:text-gray-100">
                {formatCurrency(success.total_amount, payment.currency)}
              </span>
            </div>
            <div className="flex justify-between text-sm">
              <span className="text-gray-600 dark:text-gray-400">사유</span>
              <span className="font-medium text-gray-900 dark:text-gray-100">
                {success.reason}
              </span>
            </div>
            {success.cancelled_at && (
              <div className="flex justify-between text-sm">
                <span className="text-gray-600 dark:text-gray-400">
                  처리 시각
                </span>
                <span className="font-medium text-gray-900 dark:text-gray-100">
                  {formatDate(success.cancelled_at)}
                </span>
              </div>
            )}
          </div>
          <button
            onClick={handleSuccessConfirm}
            className="w-full rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700"
          >
            확인
          </button>
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
              onChange={(e) => setReason(e.target.value)}
              placeholder={i18n.refundReasonPlaceholder}
              className="w-full rounded-md border border-gray-300 px-3 py-2 text-sm dark:border-gray-600 dark:bg-gray-700"
              rows={3}
              required
              disabled={cancelMutation.isPending}
            />
          </div>
          <div className="mb-4">
            <label className="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
              {i18n.refundAmount}
            </label>
            <input
              type="number"
              value={amount}
              onChange={(e) => setAmount(Number(e.target.value))}
              max={maxAmount}
              min={0}
              className="w-full rounded-md border border-gray-300 px-3 py-2 text-sm dark:border-gray-600 dark:bg-gray-700"
              required
              disabled={cancelMutation.isPending}
            />
            <p className="mt-1 text-xs text-gray-500 dark:text-gray-400">
              최대 환불 가능 금액: {formatCurrency(maxAmount, payment.currency)}
            </p>
          </div>
          <div className="mb-6">
            <label className="mb-1 block text-sm font-medium text-gray-700 dark:text-gray-300">
              {i18n.refundRequester}
            </label>
            <select
              value={requester}
              onChange={(e) => setRequester(e.target.value as RefundRequester)}
              className="w-full rounded-md border border-gray-300 px-3 py-2 text-sm dark:border-gray-600 dark:bg-gray-700"
              disabled={cancelMutation.isPending}
            >
              <option value={RefundRequester.Admin}>
                {i18n.refundRequesterAdmin}
              </option>
              <option value={RefundRequester.Customer}>
                {i18n.refundRequesterUser}
              </option>
            </select>
          </div>
          <div className="flex justify-end gap-3">
            <button
              type="button"
              onClick={handleClose}
              disabled={cancelMutation.isPending}
              className="rounded-md border border-gray-300 px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 disabled:cursor-not-allowed disabled:opacity-50 dark:border-gray-600 dark:text-gray-300 dark:hover:bg-gray-700"
            >
              {i18n.refundCancel}
            </button>
            <button
              type="submit"
              disabled={cancelMutation.isPending}
              className="rounded-md bg-red-600 px-4 py-2 text-sm font-medium text-white hover:bg-red-700 disabled:cursor-not-allowed disabled:opacity-50"
            >
              {cancelMutation.isPending ? '처리 중...' : i18n.refundSubmit}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
