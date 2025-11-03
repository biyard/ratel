import { useMutation } from '@tanstack/react-query';
import { useSuspenseUserInfo } from '@/hooks/use-user-info';
import * as PortOne from '@portone/browser-sdk/v2';
import { config } from '@/config';
import { useState } from 'react';
import { logger } from '@/lib/logger';
import { MembershipTier } from '@/features/membership/types/membership-tier';
import { useTranslation } from 'react-i18next';

export function useKpnPayment() {
  const { data: user } = useSuspenseUserInfo();
  const [verifying, setVerifying] = useState(false);
  const { i18n } = useTranslation();

  const mutation = useMutation({
    mutationFn: async ({
      membership,
      displayAmount,
      customerName,
      customerEmail,
      customerPhone,
    }: {
      membership: MembershipTier;
      displayAmount: number;
      customerName: string;
      customerEmail?: string;
      customerPhone?: string;
    }) => {
      let locale = PortOne.Locale.KO_KR;
      if (i18n.language === 'en') {
        locale = PortOne.Locale.EN_US;
      }

      const issueResponse = await PortOne.requestIssueBillingKey({
        storeId: config.portone_store_id,
        channelKey: config.portone_kpn_channel_key,
        displayAmount,
        currency: PortOne.Currency.USD,
        customer: {
          fullName: customerName,
          email: customerEmail,
          phoneNumber: customerPhone,
        },
        billingKeyMethod: 'CARD',
        locale,
      });

      if (issueResponse.code !== undefined) {
        return alert(issueResponse.message);
      }

      logger.debug('billing key issue response', issueResponse);

      // 고객사 서버에 빌링키를 전달합니다
      /* const response = await fetch(`${MY_SEVER_URL}/billings`, {
       *   method: 'POST',
       *   header: { 'Content-Type': 'application/json' },
       *   body: JSON.stringify({
       *     billingKey: issueResponse.billingKey,
       *     // ...
       *   }),
       * });
       * if (!response.ok) throw new Error(`response: ${await response.json()}`); */

      /* return { resp }; */
    },
    onSuccess: async () => {
      // update did
    },
  });

  return { mutation, verifying };
}
