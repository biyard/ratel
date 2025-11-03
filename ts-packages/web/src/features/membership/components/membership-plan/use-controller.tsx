import { useState } from 'react';
import { State } from '@/types/state';
import { useMembershipPlanI18n } from './i18n';
import { logger } from '@/lib/logger';
import { useKpnPayment } from '@/features/payment/hooks/use-kpn-payment';
import { MembershipTier } from '../../types/membership-tier';

export class Controller {
  constructor(
    public t: ReturnType<typeof useMembershipPlanI18n>,
    public state: State<boolean>,

    // hooks
    public kpnPayment: ReturnType<typeof useKpnPayment>,
  ) {}

  handleGetMembership = async (i: number) => {
    logger.debug('Get membership plan:', i);

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

    this.kpnPayment.mutation.mutateAsync({ membership, displayAmount });
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
  const state = useState(false);
  const kpnPayment = useKpnPayment();

  return new Controller(t, new State(state), kpnPayment);
}
