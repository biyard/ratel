'use client';

import { useState } from 'react';
import { Input } from '@/components/ui/input';
import { Col } from '@/components/ui/col';
import { Row } from '@/components/ui/row';
import { Paragraph } from '@/components/ui/paragraph';
import Heading from '@/components/ui/heading';
import { MembershipTier } from '../../types/membership-tier';
import Card from '@/components/card';

export interface MembershipPurchaseModalProps {
  membership: MembershipTier;
  displayAmount: number;
  onCancel: () => void;
  onConfirm: (customerInfo: CustomerInfo) => void;
}

export interface CustomerInfo {
  name: string;
  email: string;
  phone: string;
}

export function MembershipPurchaseModal({
  membership,
  displayAmount,
  onCancel,
  onConfirm,
}: MembershipPurchaseModalProps) {
  const [customerInfo, setCustomerInfo] = useState<CustomerInfo>({
    name: '',
    email: '',
    phone: '',
  });

  const handleSubmit = () => {
    if (!customerInfo.name || !customerInfo.email || !customerInfo.phone) {
      return;
    }
    onConfirm(customerInfo);
  };

  const isValid =
    customerInfo.name.trim() &&
    customerInfo.email.trim() &&
    customerInfo.phone.trim();

  return (
    <div className="w-[420px]">
      <Col className="gap-5">
        {/* Membership Summary */}
        <Card>
          <Row mainAxisAlignment="between" crossAxisAlignment="center">
            <Col className="gap-1">
              <Heading variant="heading5">{membership} Membership</Heading>
              <Paragraph className="text-sm text-text-secondary">
                Monthly subscription
              </Paragraph>
            </Col>
            <Heading variant="heading4" className="text-primary">
              ${displayAmount}
            </Heading>
          </Row>
        </Card>

        {/* Customer Information Form */}
        <Col className="gap-4">
          <div>
            <label className="block mb-2 text-sm font-medium text-text-primary">
              Full Name *
            </label>
            <Input
              type="text"
              placeholder="Enter your full name"
              value={customerInfo.name}
              onChange={(e) =>
                setCustomerInfo({ ...customerInfo, name: e.target.value })
              }
              data-pw="customer-name-input"
            />
          </div>

          <div>
            <label className="block mb-2 text-sm font-medium text-text-primary">
              Email Address *
            </label>
            <Input
              type="email"
              placeholder="Enter your email"
              value={customerInfo.email}
              onChange={(e) =>
                setCustomerInfo({ ...customerInfo, email: e.target.value })
              }
              data-pw="customer-email-input"
            />
          </div>
        </Col>

        {/* Footer */}
        <Row mainAxisAlignment="end" className="gap-4 mt-4">
          <button
            data-pw="purchase-cancel-button"
            onClick={onCancel}
            className="px-10 text-base font-bold transition-colors py-[14.5px] bg-cancel-button-bg text-cancel-button-text rounded-[10px] hover:text-cancel-button-text/80"
          >
            Cancel
          </button>
          <button
            data-pw="purchase-confirm-button"
            onClick={handleSubmit}
            disabled={!isValid}
            className="px-10 text-base font-bold transition-colors disabled:opacity-50 disabled:cursor-not-allowed py-[14.5px] text-submit-button-text rounded-[10px] bg-submit-button-bg hover:bg-submit-button-bg/80"
          >
            Proceed to Payment
          </button>
        </Row>
      </Col>
    </div>
  );
}
