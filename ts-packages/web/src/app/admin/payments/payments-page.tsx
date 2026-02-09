import { usePaymentsPageController } from './use-payments-page-controller';
import { useAdminPaymentsI18n } from './payments-page-i18n';
import type {
  AdminPaymentResponse,
  AdminCancelPaymentResponse,
} from '@/features/admin/types/admin-user';
import { RefundModal } from '@/features/admin/components/refund-modal';
import { PaymentsTable } from '@/features/admin/components/payments-table';
import { useState, useRef, useCallback } from 'react';
import { useCancelPaymentMutation } from '@/features/admin/hooks/use-cancel-payment-mutation';

export function PaymentsPage() {
  const ctrl = usePaymentsPageController();
  const i18n = useAdminPaymentsI18n();
  const cancelMutation = useCancelPaymentMutation();

  const [selectedPayment, setSelectedPayment] =
    useState<AdminPaymentResponse | null>(null);
  const [isRefundModalOpen, setIsRefundModalOpen] = useState(false);

  // Refund modal state
  const [reason, setReason] = useState('');
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<AdminCancelPaymentResponse | null>(
    null,
  );

  const observer = useRef<IntersectionObserver | null>(null);

  const lastPaymentRef = useCallback(
    (node: HTMLTableRowElement) => {
      if (ctrl.isFetchingNextPage) return;
      if (observer.current) observer.current.disconnect();
      observer.current = new IntersectionObserver((entries) => {
        if (entries[0].isIntersecting && ctrl.hasNextPage) {
          ctrl.fetchNextPage();
        }
      });
      if (node) observer.current.observe(node);
    },
    [ctrl],
  );

  const handleRefundClick = (payment: AdminPaymentResponse) => {
    setSelectedPayment(payment);
    setReason('');
    setError(null);
    setSuccess(null);
    setIsRefundModalOpen(true);
  };

  const handleCloseRefundModal = () => {
    setIsRefundModalOpen(false);
    setSelectedPayment(null);
    setReason('');
    setError(null);
    setSuccess(null);
  };

  const handleRefundSubmit = async () => {
    if (!selectedPayment) return;

    setError(null);

    try {
      const response = await cancelMutation.mutateAsync({
        paymentId: selectedPayment.payment_id,
        request: {
          reason,
        },
      });

      setSuccess(response);
    } catch (err) {
      setError(err instanceof Error ? err.message : i18n.refundErrorFailed);
    }
  };

  const handleSuccessConfirm = () => {
    handleCloseRefundModal();
  };

  if (ctrl.isCheckingAdmin) {
    return (
      <div className="flex h-64 items-center justify-center">
        <div className="text-gray-500 dark:text-gray-400">{i18n.loading}</div>
      </div>
    );
  }

  if (!ctrl.isAdmin) {
    return null;
  }

  if (ctrl.error) {
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

      {ctrl.isLoading ? (
        <div className="flex h-64 items-center justify-center">
          <div className="text-gray-500 dark:text-gray-400">{i18n.loading}</div>
        </div>
      ) : ctrl.payments.length === 0 ? (
        <div className="flex h-64 items-center justify-center">
          <div className="text-gray-500 dark:text-gray-400">
            {i18n.noPayments}
          </div>
        </div>
      ) : (
        <>
          <PaymentsTable
            payments={ctrl.payments}
            i18n={i18n}
            onRefundClick={handleRefundClick}
            lastPaymentRef={lastPaymentRef}
          />

          {ctrl.isFetchingNextPage && (
            <div className="mt-4 flex justify-center">
              <div className="text-gray-500 dark:text-gray-400">
                {i18n.loading}
              </div>
            </div>
          )}

          {!ctrl.hasNextPage && ctrl.payments.length > 0 && (
            <div className="mt-6 text-center text-gray-400">
              {i18n.allPaymentsLoaded}
            </div>
          )}
        </>
      )}

      <RefundModal
        isOpen={isRefundModalOpen}
        onClose={handleCloseRefundModal}
        payment={selectedPayment}
        i18n={i18n}
        reason={reason}
        error={error}
        success={success}
        isProcessing={cancelMutation.isPending}
        onReasonChange={setReason}
        onSubmit={handleRefundSubmit}
        onSuccessConfirm={handleSuccessConfirm}
      />
    </div>
  );
}
