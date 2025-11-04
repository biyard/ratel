import { useMutation } from '@tanstack/react-query';
import { useSuspenseUserInfo } from '@/hooks/use-user-info';
import * as PortOne from '@portone/browser-sdk/v2';
import { config } from '@/config';
import { call } from '@/lib/api/ratel/call';
import { useState } from 'react';
import { logger } from '@/lib/logger';
import { VerifiedCustomer } from '../types/verified_customer';

export function useIdentityVerification() {
  const { data: user } = useSuspenseUserInfo();
  const [verifying, setVerifying] = useState(false);

  const mutation = useMutation({
    mutationFn: async (): Promise<VerifiedCustomer> => {
      const identityVerificationId = `iv-${user.pk}-${crypto.randomUUID()}`;
      setVerifying(true);
      const response = await PortOne.requestIdentityVerification({
        storeId: config.portone_store_id,
        identityVerificationId,
        channelKey: config.portone_inicis_channel_key,
      });
      setVerifying(false);
      logger.debug('identity verification response', response);
      if (response.code !== undefined) {
        throw new Error('Identity verification failed');
      }

      const resp: VerifiedCustomer = await call(
        'POST',
        '/v3/payments/identify',
        {
          id: identityVerificationId,
        },
      );

      return resp;
    },
    onSuccess: async () => {
      // update did
    },
  });

  return { ...mutation, verifying };
}
