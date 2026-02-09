import { useTranslation } from 'react-i18next';

export const i18nAdminPayments = {
  en: {
    title: 'Payment History',
    loading: 'Loading payments...',
    load_error: 'Failed to load payments',
    no_payments: 'No payment records found',
    // Table headers
    payment_id: 'Payment ID',
    user_info: 'User',
    status: 'Status',
    order_name: 'Order',
    amount: 'Amount',
    paid_at: 'Date',
    subscription: 'Subscription',
    actions: 'Actions',
    // Status labels
    status_paid: 'Paid',
    status_cancelled: 'Cancelled',
    status_partial_cancelled: 'Partial Cancelled',
    status_failed: 'Failed',
    status_ready: 'Ready',
    status_virtual_account_issued: 'Virtual Account Issued',
    status_pay_pending: 'Pending',
    // Subscription
    yes: 'Yes',
    no: 'No',
    // Pagination
    prev: 'Previous',
    next: 'Next',
    page_info: 'Page {current} of {total}',
    showing: 'Showing {start}-{end} of {total}',
    // User info
    unknown_user: 'Unknown',
    no_email: 'No email',
    // Refund modal
    refund: 'Refund',
    refund_title: 'Request Refund',
    refund_reason: 'Refund Reason',
    refund_reason_placeholder: 'Enter reason for refund',
    refund_submit: 'Submit Refund Request',
    refund_cancel: 'Cancel',
    refund_success_title: 'Refund Completed',
    refund_success_confirm: 'Confirm',
    refund_cancellation_id: 'Cancellation ID',
    refund_amount_label: 'Refund Amount',
    refund_reason_label: 'Reason',
    refund_cancelled_at: 'Processed At',
    refund_processing: 'Processing...',
    refund_error_no_user: 'User information not found',
    refund_error_failed: 'Failed to process refund',
    all_payments_loaded: '🎉 All payment records loaded.',
  },
  ko: {
    title: '결제 내역',
    loading: '결제 내역 로딩 중...',
    load_error: '결제 내역 로딩 실패',
    no_payments: '결제 내역이 없습니다',
    // Table headers
    payment_id: '결제 ID',
    user_info: '사용자',
    status: '상태',
    order_name: '주문명',
    amount: '금액',
    paid_at: '결제일',
    subscription: '구독',
    actions: '관리',
    // Status labels
    status_paid: '결제완료',
    status_cancelled: '취소',
    status_partial_cancelled: '부분취소',
    status_failed: '실패',
    status_ready: '대기',
    status_virtual_account_issued: '가상계좌 발급',
    status_pay_pending: '결제대기',
    // Subscription
    yes: '예',
    no: '아니오',
    // Pagination
    prev: '이전',
    next: '다음',
    page_info: '{current} / {total} 페이지',
    showing: '총 {total}건 중 {start}-{end}',
    // User info
    unknown_user: '알 수 없음',
    no_email: '이메일 없음',
    // Refund modal
    refund: '환불',
    refund_title: '환불 요청',
    refund_reason: '취소 사유',
    refund_reason_placeholder: '취소 사유를 입력하세요',
    refund_submit: '환불 요청',
    refund_cancel: '취소',
    refund_success_title: '환불이 완료되었습니다',
    refund_success_confirm: '확인',
    refund_cancellation_id: '취소 ID',
    refund_amount_label: '환불 금액',
    refund_reason_label: '사유',
    refund_cancelled_at: '처리 시각',
    refund_processing: '처리 중...',
    refund_error_no_user: '사용자 정보를 찾을 수 없습니다.',
    refund_error_failed: '환불 처리 중 오류가 발생했습니다.',
    all_payments_loaded: '🎉 모든 결제 내역을 불러왔습니다.',
  },
};

export interface AdminPaymentsI18n {
  title: string;
  loading: string;
  loadError: string;
  noPayments: string;
  paymentId: string;
  userInfo: string;
  status: string;
  orderName: string;
  amount: string;
  paidAt: string;
  subscription: string;
  actions: string;
  statusPaid: string;
  statusCancelled: string;
  statusPartialCancelled: string;
  statusFailed: string;
  statusReady: string;
  statusVirtualAccountIssued: string;
  statusPayPending: string;
  yes: string;
  no: string;
  prev: string;
  next: string;
  pageInfo: string;
  showing: string;
  unknownUser: string;
  noEmail: string;
  // Refund modal
  refund: string;
  refundTitle: string;
  refundReason: string;
  refundReasonPlaceholder: string;
  refundSubmit: string;
  refundCancel: string;
  refundSuccessTitle: string;
  refundSuccessConfirm: string;
  refundCancellationId: string;
  refundAmountLabel: string;
  refundReasonLabel: string;
  refundCancelledAt: string;
  refundProcessing: string;
  refundErrorNoUser: string;
  refundErrorFailed: string;
  allPaymentsLoaded: string;
}

export function useAdminPaymentsI18n(): AdminPaymentsI18n {
  const { t } = useTranslation('AdminPayments');

  return {
    title: t('title'),
    loading: t('loading'),
    loadError: t('load_error'),
    noPayments: t('no_payments'),
    paymentId: t('payment_id'),
    userInfo: t('user_info'),
    status: t('status'),
    orderName: t('order_name'),
    amount: t('amount'),
    paidAt: t('paid_at'),
    subscription: t('subscription'),
    actions: t('actions'),
    statusPaid: t('status_paid'),
    statusCancelled: t('status_cancelled'),
    statusPartialCancelled: t('status_partial_cancelled'),
    statusFailed: t('status_failed'),
    statusReady: t('status_ready'),
    statusVirtualAccountIssued: t('status_virtual_account_issued'),
    statusPayPending: t('status_pay_pending'),
    yes: t('yes'),
    no: t('no'),
    prev: t('prev'),
    next: t('next'),
    pageInfo: t('page_info'),
    showing: t('showing'),
    unknownUser: t('unknown_user'),
    noEmail: t('no_email'),
    // Refund modal
    refund: t('refund'),
    refundTitle: t('refund_title'),
    refundReason: t('refund_reason'),
    refundReasonPlaceholder: t('refund_reason_placeholder'),
    refundSubmit: t('refund_submit'),
    refundCancel: t('refund_cancel'),
    refundSuccessTitle: t('refund_success_title'),
    refundSuccessConfirm: t('refund_success_confirm'),
    refundCancellationId: t('refund_cancellation_id'),
    refundAmountLabel: t('refund_amount_label'),
    refundReasonLabel: t('refund_reason_label'),
    refundCancelledAt: t('refund_cancelled_at'),
    refundProcessing: t('refund_processing'),
    refundErrorNoUser: t('refund_error_no_user'),
    refundErrorFailed: t('refund_error_failed'),
    allPaymentsLoaded: t('all_payments_loaded'),
  };
}
