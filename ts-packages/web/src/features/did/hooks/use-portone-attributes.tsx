import { useMutation } from '@tanstack/react-query';
import { useSuspenseUserInfo } from '@/hooks/use-user-info';
import * as PortOne from '@portone/browser-sdk/v2';
import { config } from '@/config';
import { call } from '@/lib/api/ratel/call';
import { useState } from 'react';
import { logger } from '@/lib/logger';
import { VerifiedAttributes } from '../types/verified_attributes';
import { optimisticUpdate } from '@/lib/hook-utils';
import { UserAttributes } from '../types/user-attributes';

export function usePortOneAttributes() {
  const { data: user } = useSuspenseUserInfo();
  const [verifying, setVerifying] = useState(false);

  const mutation = useMutation({
    mutationFn: async () => {
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

      const resp = await call('PUT', '/v3/me/did', {
        type: 'port_one',
        id: identityVerificationId,
      });

      return { attributes: new VerifiedAttributes(resp) };
    },
    onSuccess: async ({ attributes }) => {
      optimisticUpdate<UserAttributes>(
        { queryKey: ['user-verified-attributes'] },
        (old) => {
          old.age = attributes.age;
          old.gender = attributes.gender;

          return old;
        },
      );
      // update did
    },
  });

  return { ...mutation, verifying };
}
