'use client';

import Loading from '@/app/loading';
import { subscribeRequest } from '@/lib/api/models/telegrams/subscribe';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import { useSearchParams } from 'next/navigation';
import { useEffect, useRef } from 'react';
import { postEvent } from '@telegram-apps/sdk';

export default function SubscribePage() {
  const searchParams = useSearchParams();
  const isSubscribing = useRef(false);

  const { post } = useApiCall();

  useEffect(() => {
    const subscribe = async () => {
      if (isSubscribing.current) {
        return;
      }

      isSubscribing.current = true;

      const chatId = searchParams.get('chat_id');
      if (!chatId) {
        console.error('chat_id parameter is required');
        return;
      }

      const chatIdNum = Number(chatId);
      const lang = searchParams.get('lang');

      try {
        console.log('Starting subscription...');
        await post(
          ratelApi.telegram.subscribe(),
          subscribeRequest(chatIdNum, lang ?? undefined),
        );
        console.log('Subscription completed');
      } catch (error) {
        console.error('Error subscribing to Telegram:', error);
      } finally {
        postEvent('web_app_close');
      }
    };

    subscribe();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);
  return <Loading />;
}
