import { useMutation } from '@tanstack/react-query';
import { useSuspenseUserInfo } from '@/hooks/use-user-info';
import * as PortOne from '@portone/browser-sdk/v2';
import { config } from '@/config';
import { useState } from 'react';
import { logger } from '@/lib/logger';
import { MembershipTier } from '@/features/membership/types/membership-tier';
import { useTranslation } from 'react-i18next';
import { call } from '@/lib/api/ratel/call';

export function useKpnPayment() {
  const { data: user } = useSuspenseUserInfo();
  const [verifying, setVerifying] = useState(false);
  const { i18n } = useTranslation();

  const mutation = useMutation({
    mutationFn: async ({
      membership,
      cardNumber,
      expiryYear,
      expiryMonth,
      birthOrBusinessRegistrationNumber,
      passwordTwoDigits,
    }: {
      membership: MembershipTier;
      cardNumber: string;
      expiryYear: string;
      expiryMonth: string;
      birthOrBusinessRegistrationNumber: string;
      passwordTwoDigits: string;
    }) => {
      const _resp = await call('POST', '/v3/payments/memberships', {
        membership,
        card_number: cardNumber,
        expiry_year: expiryYear,
        expiry_month: expiryMonth,
        birth_or_business_registration_number:
          birthOrBusinessRegistrationNumber,
        password_two_digits: passwordTwoDigits,
      });
    },
    onSuccess: async () => {
      // update did
    },
  });

  return { mutation, verifying };
}
