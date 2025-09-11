'use client';

import { useRouter } from 'next/navigation';
import { route } from '@/route';
import { Button } from '@/components/ui/button';
import { TelegramIcon } from '@/components/icons';
import { useTranslations } from 'next-intl';
import { useState } from 'react';
import { ratelApi } from '@/lib/api/ratel_api';
import { config } from '@/config';
import { apiFetch } from '@/lib/api/apiFetch';
import { isTMA, postEvent } from '@telegram-apps/sdk-react';
import { Service, useAuthStore } from '../store';

function updateTelegramId(token: string) {
  return apiFetch<void>(
    `${config.api_url}${ratelApi.users.updateTelegramId()}`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        token,
      }),
    },
  );
}

export default function Client() {
  const { redirectUrl, service, token } = useAuthStore();

  const t = useTranslations('Connect');
  const [isLoading, setIsLoading] = useState(false);

  const router = useRouter();
  if (!service) {
    router.push(route.home());
    return null;
  }

  const handleConfirm = async () => {
    setIsLoading(true);
    try {
      switch (service) {
        case Service.Telegram:
          if (!token) {
            throw new Error('Missing token for linking Telegram account');
          }
          await updateTelegramId(token);
          break;
      }
      if (redirectUrl) {
        // const isIOSTelegramInAppBrowser =
        //   navigator.userAgent.includes('iPhone') &&
        //   // eslint-disable-next-line @typescript-eslint/no-explicit-any
        //   typeof (window as any).TelegramWebviewProxy !== 'undefined' &&
        //   // eslint-disable-next-line @typescript-eslint/no-explicit-any
        //   typeof (window as any).TelegramWebviewProxyProto !== 'undefined';

        const isAndroidTelegramInAppBrowser =
          navigator.userAgent.includes('Android') &&
          // eslint-disable-next-line @typescript-eslint/no-explicit-any
          typeof (window as any).TelegramWebview !== 'undefined';

        if (isAndroidTelegramInAppBrowser) {
          //redirectUrl : tg://resolve?domain={BOT_NAME}/app?startapp={PARAM}
          //FIXME: In Android Telegram In-App Browser, open redirect url is not working.
          postEvent('web_app_close', {});
          window.close();
        } else {
          window.open(redirectUrl, '_blank', 'noopener,noreferrer');
          setTimeout(() => {
            window.close();
          }, 100);
        }
      } else {
        window.close();
      }
    } catch (error) {
      alert(error);
      console.error('Failed to link Telegram account:', error);
    } finally {
      setIsLoading(false);
    }
  };

  const handleCancel = () => {
    if (isTMA()) {
      window.close();
      return;
    }
    router.push(route.home());
  };

  return (
    <div className="flex flex-col items-center justify-center w-full max-w-md p-8 mx-auto mt-10 border rounded-lg shadow-lg">
      <TelegramIcon width={60} height={60} className="mb-4 text-blue-500" />
      <h1 className="text-2xl font-bold text-center mb-2">
        {t('title', { service: service })}
      </h1>
      <p className="text-center text-gray-600 mb-8">
        {t('description', { service: service })}
      </p>

      <div className="flex flex-col w-full gap-4">
        <Button
          variant="rounded_primary"
          onClick={async () => {
            await handleConfirm();
          }}
          disabled={isLoading}
        >
          {isLoading ? t('ConfirmButton.loading') : t('ConfirmButton.label')}
        </Button>
        <Button
          variant="rounded_secondary"
          onClick={handleCancel}
          disabled={isLoading}
        >
          {t('CancelButton.label')}
        </Button>
      </div>
    </div>
  );
}
