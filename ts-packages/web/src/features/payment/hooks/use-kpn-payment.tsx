import { useMutation } from '@tanstack/react-query';
import { MembershipTier } from '@/features/membership/types/membership-tier';
import { call } from '@/lib/api/ratel/call';

export interface PaymentReceipt {
  status: string;
  transaction_id: string;
  membership_tier: string;
  amount: number;
  duration_days: number;
  credits: number;
  paid_at: number;
}

export function useKpnPayment() {
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
    }): Promise<PaymentReceipt> => {
      const resp = await call('POST', '/v3/payments/memberships', {
        membership,
        card_number: cardNumber,
        expiry_year: expiryYear,
        expiry_month: expiryMonth,
        birth_or_business_registration_number:
          birthOrBusinessRegistrationNumber,
        password_two_digits: passwordTwoDigits,
      });

      return resp as PaymentReceipt;
    },
    onSuccess: async () => {
      // update did
    },
  });

  return { mutation };
}
