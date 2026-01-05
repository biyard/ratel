import { useState } from 'react';
import { State } from '@/types/state';
import { useMembershipPlanI18n } from './i18n';
import { logger } from '@/lib/logger';
import { useKpnPayment } from '@/features/payment/hooks/use-kpn-payment';
import { MembershipTier } from '../../types/membership-tier';
import { usePopup } from '@/lib/contexts/popup-service';
import {
  CustomerInfo,
  MembershipPurchaseModal,
} from './membership-purchase-modal';
import { MembershipReceiptModal } from './membership-receipt-modal';
import { useIdentityVerification } from '@/features/did/hooks/use-identity-verification';
import { LoginModal } from '@/components/popup/login-popup';
import { useUserInfo } from '@/hooks/use-user-info';
import { useTranslation } from 'react-i18next';

export class Controller {
  constructor(
    public t: ReturnType<typeof useMembershipPlanI18n>,
    public state: State<boolean>,

    // hooks
    public user: ReturnType<typeof useUserInfo>,
    public kpnPayment: ReturnType<typeof useKpnPayment>,
    public popup: ReturnType<typeof usePopup>,
    public verification: ReturnType<typeof useIdentityVerification>,
    public i18n: ReturnType<typeof useTranslation>['i18n'],
  ) {}

  openLoginModal = () => {
    this.popup
      .open(<LoginModal />)
      .withTitle('Join the Movement')
      .withoutBackdropClose();
  };

  handleGetMembership = async (i: number) => {
    logger.debug('Get membership plan:', i);
    if (!this.user.data) {
      return this.openLoginModal();
    }

    let membership = MembershipTier.Pro;
    let displayAmount = 20;

    if (i === 2) {
      membership = MembershipTier.Max;
      displayAmount = 50;
    } else if (i === 3) {
      membership = MembershipTier.Vip;
      displayAmount = 100;
    } else if (i === 1) {
      membership = MembershipTier.Pro;
      displayAmount = 20;
    } else {
      return this.handleEnterpriseContact();
    }

    if (this.i18n.language === 'ko') {
      displayAmount = Math.round(displayAmount * 1500);
    }

    if (this.user.data.has_billing_key) {
      const receipt = await this.kpnPayment.mutation.mutateAsync({
        membership,
      });

      if (!receipt.receipt) {
        // TODO: handle downgrade membership
        return;
      }

      // Show receipt modal
      this.popup
        .open(
          <MembershipReceiptModal
            receipt={receipt}
            onClose={() => {
              this.popup.close();
            }}
            t={this.t.receiptModal}
          />,
        )
        .withTitle('Receipt');

      return;
    }

    try {
      const resp = await this.verification.mutateAsync();
      logger.debug('Identity verification successful:', resp);

      this.popup
        .open(
          <MembershipPurchaseModal
            membership={membership}
            customer={resp}
            displayAmount={displayAmount}
            t={this.t.purchaseModal}
            onCancel={() => {
              logger.debug('Membership purchase cancelled');
              this.popup.close();
            }}
            onConfirm={async (cardinfo: CustomerInfo) => {
              logger.debug('Membership purchase confirmed:', cardinfo);
              this.popup.close();
              const receipt = await this.kpnPayment.mutation.mutateAsync({
                membership,
                cardNumber: cardinfo.cardNumber,
                expiryYear: cardinfo.expiryYear,
                expiryMonth: cardinfo.expiryMonth,
                birthOrBusinessRegistrationNumber: cardinfo.birthOrBiz,
                passwordTwoDigits: cardinfo.cardPassword,
              });

              // Show receipt modal
              this.popup
                .open(
                  <MembershipReceiptModal
                    receipt={receipt}
                    onClose={() => {
                      this.popup.close();
                    }}
                    t={this.t.receiptModal}
                  />,
                )
                .withTitle('Receipt');
            }}
          />,
        )
        .withoutClose();
    } catch (e) {
      logger.error('Identity verification failed:', e);
    }
  };

  handleEnterpriseContact = () => {
    const email = 'hi@ratel.foundation';
    const subject = encodeURIComponent('Enterprise Membership Inquiry');
    const body = encodeURIComponent(
      'Hello,\n\nI would like to learn more about the Enterprise membership plan.\n\nThank you.',
    );

    const mailtoUrl = `mailto:${email}?subject=${subject}&body=${body}`;
    window.open(mailtoUrl, '_blank');

    logger.debug('Opening email client for enterprise contact:', email);
  };
}

export function useController() {
  // TODO: use or define hooks
  const t = useMembershipPlanI18n();
  const user = useUserInfo();
  const state = useState(false);
  const kpnPayment = useKpnPayment();
  const popup = usePopup();
  const verification = useIdentityVerification();
  const { i18n } = useTranslation();

  return new Controller(
    t,
    new State(state),
    user,
    kpnPayment,
    popup,
    verification,
    i18n,
  );
}
