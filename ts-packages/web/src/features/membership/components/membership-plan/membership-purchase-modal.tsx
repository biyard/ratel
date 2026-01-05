'use client';

import { useState } from 'react';
import { Input } from '@/components/ui/input';
import { Col } from '@/components/ui/col';
import { Row } from '@/components/ui/row';
import { Paragraph } from '@/components/ui/paragraph';
import Heading from '@/components/ui/heading';
import { MembershipTier } from '../../types/membership-tier';
import Card from '@/components/card';
import { PurchaseModalI18n } from './i18n';
import { VerifiedCustomer } from '@/features/did/types/verified_customer';
import { useTranslation } from 'react-i18next';

export interface MembershipPurchaseModalProps {
  membership: MembershipTier;
  displayAmount: number;
  onCancel: () => void;
  onConfirm: (customerInfo: CustomerInfo) => void;
  customer: VerifiedCustomer;
  t: PurchaseModalI18n;
}

export interface CustomerInfo {
  name: string;
  cardNumber: string;
  expiryMonth: string;
  expiryYear: string;
  birthOrBiz: string;
  cardPassword: string;
}

export function MembershipPurchaseModal({
  membership,
  displayAmount,
  onCancel,
  onConfirm,
  customer,
  t,
}: MembershipPurchaseModalProps) {
  let isBusiness = false;
  if (customer.birthDate.split('-')[0].length == 3) {
    isBusiness = true;
  }

  const [customerInfo, setCustomerInfo] = useState<CustomerInfo>({
    name: customer.name,
    cardNumber: '',
    expiryMonth: '',
    expiryYear: '',
    birthOrBiz: isBusiness
      ? customer.birthDate.replaceAll('-', '').slice(0, 10)
      : customer.birthDate.replaceAll('-', '').slice(2, 8),
    cardPassword: '',
  });

  const { i18n } = useTranslation();

  const handleSubmit = () => {
    if (
      !customerInfo.name ||
      !customerInfo.cardNumber ||
      !customerInfo.expiryMonth ||
      !customerInfo.expiryYear ||
      !customerInfo.birthOrBiz ||
      !customerInfo.cardPassword
    ) {
      return;
    }
    onConfirm(customerInfo);
  };

  const isValid =
    customerInfo.name.trim() &&
    customerInfo.cardNumber.trim() &&
    customerInfo.expiryMonth.trim() &&
    customerInfo.expiryYear.trim() &&
    customerInfo.birthOrBiz.trim() &&
    customerInfo.cardPassword.trim();

  return (
    <div className="w-[420px]">
      <Col className="gap-5">
        {/* Membership Summary */}
        <Card>
          <Row mainAxisAlignment="between" crossAxisAlignment="center">
            <Col className="gap-1">
              <Heading variant="heading5">
                {membership} {t.membershipLabel}
              </Heading>
              <Paragraph className="text-sm text-text-secondary">
                {t.monthlySubscription}
              </Paragraph>
            </Col>
            <Heading variant="heading4" className="text-primary">
              {i18n.language === 'ko' ? '₩' : '$'}
              {displayAmount}
            </Heading>
          </Row>
        </Card>

        {/* Customer Information Form */}
        <Col className="gap-4">
          <div>
            <label className="block mb-2 text-sm font-medium text-text-primary">
              {t.fullNameLabel}
            </label>
            <Input type="text" value={customer.name} disabled />
          </div>

          {/* Card Information */}
          <div className="pt-4">
            <Col className="gap-4">
              <div>
                <label className="block mb-2 text-sm font-medium text-text-primary">
                  {t.cardNumberLabel}
                </label>
                <Input
                  type="text"
                  placeholder={t.cardNumberPlaceholder}
                  value={customerInfo.cardNumber}
                  onChange={(e) =>
                    setCustomerInfo({
                      ...customerInfo,
                      cardNumber: e.target.value.replace(/\D/g, ''),
                    })
                  }
                  maxLength={16}
                />
              </div>

              <div>
                <label className="block mb-2 text-sm font-medium text-text-primary">
                  {t.expiryLabel}
                </label>
                <Row className="gap-2">
                  <Input
                    type="text"
                    placeholder={t.expiryMonthPlaceholder}
                    value={customerInfo.expiryMonth}
                    onChange={(e) =>
                      setCustomerInfo({
                        ...customerInfo,
                        expiryMonth: e.target.value.replace(/\D/g, ''),
                      })
                    }
                    maxLength={2}
                    className="flex-1"
                  />
                  <Input
                    type="text"
                    placeholder={t.expiryYearPlaceholder}
                    value={customerInfo.expiryYear}
                    onChange={(e) =>
                      setCustomerInfo({
                        ...customerInfo,
                        expiryYear: e.target.value.replace(/\D/g, ''),
                      })
                    }
                    maxLength={2}
                    className="flex-1"
                  />
                </Row>
              </div>

              <div>
                <label className="block mb-2 text-sm font-medium text-text-primary">
                  {t.birthOrBizLabel}
                </label>
                <Input
                  type="text"
                  placeholder={t.birthOrBizPlaceholder}
                  value={customerInfo.birthOrBiz}
                  onChange={(e) =>
                    setCustomerInfo({
                      ...customerInfo,
                      birthOrBiz: e.target.value.replace(/\D/g, ''),
                    })
                  }
                  maxLength={10}
                />
              </div>

              <div>
                <label className="block mb-2 text-sm font-medium text-text-primary">
                  {t.cardPasswordLabel}
                </label>
                <Input
                  type="password"
                  placeholder={t.cardPasswordPlaceholder}
                  value={customerInfo.cardPassword}
                  onChange={(e) =>
                    setCustomerInfo({
                      ...customerInfo,
                      cardPassword: e.target.value.replace(/\D/g, ''),
                    })
                  }
                  maxLength={2}
                  className="w-20"
                />
              </div>
            </Col>
          </div>
        </Col>

        {/* Footer */}
        <Row mainAxisAlignment="end" className="gap-4 mt-4">
          <button
            data-pw="purchase-cancel-button"
            onClick={onCancel}
            className="px-10 text-base font-bold transition-colors py-[14.5px] bg-cancel-button-bg text-cancel-button-text rounded-[10px] hover:text-cancel-button-text/80"
          >
            {t.cancelButton}
          </button>
          <button
            data-pw="purchase-confirm-button"
            onClick={handleSubmit}
            disabled={!isValid}
            className="px-10 text-base font-bold transition-colors disabled:opacity-50 disabled:cursor-not-allowed py-[14.5px] text-submit-button-text rounded-[10px] bg-submit-button-bg hover:bg-submit-button-bg/80"
          >
            {t.confirmButton}
          </button>
        </Row>
      </Col>
    </div>
  );
}
