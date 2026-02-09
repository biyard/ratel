import { usePaymentsPageController } from './use-payments-page-controller';
import { useAdminPaymentsI18n, AdminPaymentsI18n } from './payments-page-i18n';
import type { AdminPaymentResponse } from '@/features/admin/types/admin-user';
import { RefundRequester } from '@/features/admin/types/admin-user';
import { useState, useEffect, useRef, useCallback } from 'react';

function getStatusLabel(status: string, i18n: AdminPaymentsI18n): string {
  const statusMap: Record<string, string> = {
    PAID: i18n.statusPaid,
    CANCELLED: i18n.statusCancelled,
    PARTIAL_CANCELLED: i18n.statusPartialCancelled,
    FAILED: i18n.statusFailed,
    READY: i18n.statusReady,
    VIRTUAL_ACCOUNT_ISSUED: i18n.statusVirtualAccountIssued,
    PAY_PENDING: i18n.statusPayPending,
  };
  return statusMap[status] || status;
}

function getStatusColor(status: string): string {
  const colorMap: Record<string, string> = {
    PAID: 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300',
    CANCELLED: 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300',
    PARTIAL_CANCELLED:
      'bg-orange-100 text-orange-800 dark:bg-orange-900 dark:text-orange-300',
    FAILED: 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300',
    READY: 'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-300',
    VIRTUAL_ACCOUNT_ISSUED:
      'bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-300',
    PAY_PENDING:
      'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-300',
  };
  return (
    colorMap[status] ||
    'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300'
  );
}

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

function RefundModal({ isOpen, onClose, payment, i18n }: RefundModalProps) {
  const [reason, setReason] = useState('');
  const [amount, setAmount] = useState(0);
  const [requester, setRequester] = useState<RefundRequester>(
    RefundRequester.Admin,
  );

  // payment가 변경될 때 amount를 최대값(원래 금액)으로 설정
  useEffect(() => {
    if (payment) {
      setAmount(payment.total);
    }
  }, [payment]);

  if (!isOpen || !payment) return null;

  const maxAmount = payment.total;

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    // TODO: Implement refund API call
    console.log('Refund request:', {
      payment_id: payment.payment_id,
      reason,
      amount,
      requester,
    });
    onClose();
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div className="w-full max-w-md rounded-lg bg-white p-6 shadow-xl dark:bg-gray-800">
        <h2 className="mb-4 text-xl font-semibold text-gray-900 dark:text-white">
          {i18n.refundTitle}
        </h2>
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
            >
              <option value={RefundRequester.Admin}>
                {i18n.refundRequesterAdmin}
              </option>
              <option value={RefundRequester.User}>
                {i18n.refundRequesterUser}
              </option>
            </select>
          </div>
          <div className="flex justify-end gap-3">
            <button
              type="button"
              onClick={onClose}
              className="rounded-md border border-gray-300 px-4 py-2 text-sm font-medium text-gray-700 hover:bg-gray-50 dark:border-gray-600 dark:text-gray-300 dark:hover:bg-gray-700"
            >
              {i18n.refundCancel}
            </button>
            <button
              type="submit"
              className="rounded-md bg-red-600 px-4 py-2 text-sm font-medium text-white hover:bg-red-700"
            >
              {i18n.refundSubmit}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}

export function PaymentsPage() {
  const i18n = useAdminPaymentsI18n();
  const controller = usePaymentsPageController();
  const [selectedPayment, setSelectedPayment] =
    useState<AdminPaymentResponse | null>(null);
  const [isRefundModalOpen, setIsRefundModalOpen] = useState(false);
  const observer = useRef<IntersectionObserver | null>(null);

  const lastPaymentRef = useCallback(
    (node: HTMLTableRowElement) => {
      if (controller.isFetchingNextPage) return;
      if (observer.current) observer.current.disconnect();
      observer.current = new IntersectionObserver((entries) => {
        if (entries[0].isIntersecting && controller.hasNextPage) {
          controller.fetchNextPage();
        }
      });
      if (node) observer.current.observe(node);
    },
    [
      controller.isFetchingNextPage,
      controller.fetchNextPage,
      controller.hasNextPage,
    ],
  );

  const handleRefundClick = (payment: AdminPaymentResponse) => {
    setSelectedPayment(payment);
    setIsRefundModalOpen(true);
  };

  const handleCloseRefundModal = () => {
    setIsRefundModalOpen(false);
    setSelectedPayment(null);
  };

  if (controller.isCheckingAdmin) {
    return (
      <div className="flex h-64 items-center justify-center">
        <div className="text-gray-500 dark:text-gray-400">{i18n.loading}</div>
      </div>
    );
  }

  if (!controller.isAdmin) {
    return null;
  }

  if (controller.error) {
    return (
      <div className="flex h-64 items-center justify-center">
        <div className="text-red-500">{i18n.loadError}</div>
      </div>
    );
  }

  return (
    <div className="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
      <div className="mb-6">
        <h1 className="text-2xl font-bold text-gray-900 dark:text-white">
          {i18n.title}
        </h1>
      </div>

      {controller.isLoading ? (
        <div className="flex h-64 items-center justify-center">
          <div className="text-gray-500 dark:text-gray-400">{i18n.loading}</div>
        </div>
      ) : controller.payments.length === 0 ? (
        <div className="flex h-64 items-center justify-center">
          <div className="text-gray-500 dark:text-gray-400">
            {i18n.noPayments}
          </div>
        </div>
      ) : (
        <>
          <div className="overflow-x-auto rounded-lg border border-gray-200 dark:border-gray-700">
            <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
              <thead className="bg-gray-50 dark:bg-gray-800">
                <tr>
                  <th className="px-4 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">
                    {i18n.userInfo}
                  </th>
                  <th className="px-4 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">
                    {i18n.status}
                  </th>
                  <th className="px-4 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">
                    {i18n.orderName}
                  </th>
                  <th className="px-4 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">
                    {i18n.amount}
                  </th>
                  <th className="px-4 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">
                    {i18n.paidAt}
                  </th>
                  <th className="px-4 py-3 text-left text-xs font-medium uppercase tracking-wider text-gray-500 dark:text-gray-400">
                    {i18n.actions}
                  </th>
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-200 bg-white dark:divide-gray-700 dark:bg-gray-900">
                {controller.payments.map((payment, index) => (
                  <tr
                    key={payment.payment_id}
                    ref={
                      index === controller.payments.length - 1
                        ? lastPaymentRef
                        : null
                    }
                    className="hover:bg-gray-50 dark:hover:bg-gray-800"
                  >
                    <td className="px-4 py-3 text-sm">
                      <div className="text-gray-900 dark:text-gray-100">
                        {payment.user_name || i18n.unknownUser}
                      </div>
                      <div className="text-xs text-gray-500 dark:text-gray-400">
                        {payment.user_email || i18n.noEmail}
                      </div>
                    </td>
                    <td className="whitespace-nowrap px-4 py-3 text-sm">
                      <span
                        className={`inline-flex rounded-full px-2 py-1 text-xs font-semibold ${getStatusColor(payment.status)}`}
                      >
                        {getStatusLabel(payment.status, i18n)}
                      </span>
                    </td>
                    <td className="px-4 py-3 text-sm text-gray-900 dark:text-gray-100">
                      {payment.order_name}
                    </td>
                    <td className="whitespace-nowrap px-4 py-3 text-sm font-medium text-gray-900 dark:text-gray-100">
                      {formatCurrency(payment.total, payment.currency)}
                    </td>
                    <td className="whitespace-nowrap px-4 py-3 text-sm text-gray-500 dark:text-gray-400">
                      {formatDate(payment.paid_at)}
                    </td>
                    <td className="whitespace-nowrap px-4 py-3 text-sm">
                      <button
                        onClick={() => handleRefundClick(payment)}
                        disabled={
                          payment.status === 'CANCELLED' ||
                          payment.status === 'FAILED'
                        }
                        className="rounded bg-red-100 px-3 py-1 text-xs font-medium text-red-700 hover:bg-red-200 disabled:cursor-not-allowed disabled:opacity-50 dark:bg-red-900 dark:text-red-300 dark:hover:bg-red-800"
                      >
                        {i18n.refund}
                      </button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>

          {controller.isFetchingNextPage && (
            <div className="mt-4 flex justify-center">
              <div className="text-gray-500 dark:text-gray-400">
                {i18n.loading}
              </div>
            </div>
          )}

          {!controller.hasNextPage && controller.payments.length > 0 && (
            <div className="mt-6 text-center text-gray-400">
              🎉 모든 결제 내역을 불러왔습니다.
            </div>
          )}
        </>
      )}

      <RefundModal
        isOpen={isRefundModalOpen}
        onClose={handleCloseRefundModal}
        payment={selectedPayment}
        i18n={i18n}
      />
    </div>
  );
}
