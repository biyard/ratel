import { useMutation } from '@tanstack/react-query';
import { MembershipTier } from '@/features/membership/types/membership-tier';
import { call } from '@/lib/api/ratel/call';
import { useTranslation } from 'react-i18next';
import { MembershipResponse } from '@/features/membership/dto/membership-response';

export interface MembershipPaymentResponse {
  renewal_date: number;
  receipt?: PaymentReceipt;
  membership: MembershipResponse;
}

export interface PaymentReceipt {
  id: string;
  paid_at: number;
  tx_type: string; // PURCHASE__MEMBERSHIP#PRO etc.
  currency: 'KRW' | 'USD';
  tx_id: string;
  amount: number;
}

export function useKpnPayment() {
  const { i18n } = useTranslation();
  const currency = i18n.language === 'ko' ? 'KRW' : 'USD';

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
      cardNumber?: string;
      expiryYear?: string;
      expiryMonth?: string;
      birthOrBusinessRegistrationNumber?: string;
      passwordTwoDigits?: string;
    }): Promise<MembershipPaymentResponse> => {
      const card_info = cardNumber
        ? {
            card_number: cardNumber,
            expiry_year: expiryYear,
            expiry_month: expiryMonth,
            birth_or_business_registration_number:
              birthOrBusinessRegistrationNumber,
            password_two_digits: passwordTwoDigits,
          }
        : null;

      const resp: MembershipPaymentResponse = await call(
        'POST',
        '/v3/me/memberships',
        {
          membership,
          currency,
          card_info,
        },
      );

      resp.membership = new MembershipResponse(resp.membership);

      return resp;
    },
    onSuccess: async () => {
      // update did
    },
  });

  return { mutation };
}
