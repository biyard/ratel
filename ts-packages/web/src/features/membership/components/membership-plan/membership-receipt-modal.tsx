'use client';

import { Col } from '@/components/ui/col';
import { Row } from '@/components/ui/row';
import { Paragraph } from '@/components/ui/paragraph';
import Heading from '@/components/ui/heading';
import Card from '@/components/card';
import { ReceiptModalI18n } from './i18n';
import { MembershipPaymentResponse } from '@/features/payment/hooks/use-kpn-payment';

export interface MembershipReceiptModalProps {
  receipt: MembershipPaymentResponse;
  onClose: () => void;
  t: ReceiptModalI18n;
}

export function MembershipReceiptModal({
  receipt,
  onClose,
  t,
}: MembershipReceiptModalProps) {
  // Convert microseconds to date string
  const formatDate = (microseconds: number) => {
    const date = new Date(microseconds / 1000);
    return date.toLocaleDateString(undefined, {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
    });
  };

  return (
    <div className="w-[420px]">
      <Col className="gap-5">
        {/* Success Header */}
        <div className="text-center">
          <div className="mb-4 text-6xl">✓</div>
          <Heading variant="heading4" className="mb-2 text-primary">
            {t.title}
          </Heading>
          <Paragraph className="text-text-secondary">
            {t.thankYouMessage}
          </Paragraph>
        </div>

        {/* Receipt Details */}
        <Card>
          <Col className="gap-4">
            {/* Transaction ID */}
            <Row mainAxisAlignment="between" crossAxisAlignment="center">
              <Paragraph className="text-sm font-medium text-text-secondary">
                {t.transactionIdLabel}
              </Paragraph>
              <Paragraph className="font-mono text-sm text-text-primary">
                {receipt.receipt.tx_id.slice(0, 16)}...
              </Paragraph>
            </Row>

            <div className="h-px bg-border" />

            {/* Membership Tier */}
            <Row mainAxisAlignment="between" crossAxisAlignment="center">
              <Paragraph className="text-sm font-medium text-text-secondary">
                {t.membershipLabel}
              </Paragraph>
              <Paragraph className="text-sm font-semibold text-text-primary">
                {receipt.membership.tier}
              </Paragraph>
            </Row>

            {/* Amount */}
            <Row mainAxisAlignment="between" crossAxisAlignment="center">
              <Paragraph className="text-sm font-medium text-text-secondary">
                {t.amountLabel}
              </Paragraph>
              <Heading variant="heading5" className="text-primary">
                ${receipt.receipt.amount}
              </Heading>
            </Row>

            {/* Duration */}
            <Row mainAxisAlignment="between" crossAxisAlignment="center">
              <Paragraph className="text-sm font-medium text-text-secondary">
                {t.durationLabel}
              </Paragraph>
              <Paragraph className="text-sm text-text-primary">
                {receipt.membership.duration_days} {t.daysLabel}
              </Paragraph>
            </Row>

            {/* Credits */}
            <Row mainAxisAlignment="between" crossAxisAlignment="center">
              <Paragraph className="text-sm font-medium text-text-secondary">
                {t.creditsLabel}
              </Paragraph>
              <Paragraph className="text-sm text-text-primary">
                {receipt.membership.credits.toLocaleString()}
              </Paragraph>
            </Row>

            <div className="h-px bg-border" />

            {/* Payment Date */}
            <Row mainAxisAlignment="between" crossAxisAlignment="center">
              <Paragraph className="text-sm font-medium text-text-secondary">
                {t.paidAtLabel}
              </Paragraph>
              <Paragraph className="text-sm text-text-primary">
                {formatDate(receipt.receipt.paid_at)}
              </Paragraph>
            </Row>
          </Col>
        </Card>

        {/* Close Button */}
        <button
          data-pw="receipt-close-button"
          onClick={onClose}
          className="px-10 w-full text-base font-bold transition-colors py-[14.5px] text-submit-button-text rounded-[10px] bg-submit-button-bg hover:bg-submit-button-bg/80"
        >
          {t.closeButton}
        </button>
      </Col>
    </div>
  );
}
