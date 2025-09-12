'use client';

import { useEffect, useState } from 'react';
import { useRawInitData, postEvent } from '@telegram-apps/sdk-react';
import { useAuth } from '@/lib/contexts/auth-context';
import { proxy, ratelApi } from '@/lib/api/ratel_api';
import { send } from '@/lib/api/send';

import Loading from '../loading';
import { useRouter, useSearchParams } from 'next/navigation';
import { decode_base64 } from '@/lib/base64';
import { route } from '@/route';
import { TelegramWebCommand, TgWebParams } from '@/types/telegram';
import { Button } from '@/components/ui/button';
import { config } from '@/config';
import { apiFetch } from '@/lib/api/apiFetch';

function useDidMount(): boolean {
  const [didMount, setDidMount] = useState<boolean>(false);

  useEffect(() => {
    setDidMount(true);
  }, []);

  return didMount;
}

export default function HomePage() {
  const didMount = useDidMount();
  return didMount && <TelegramMiniAppMain />;
}

function parseTelegramStartParam(startParam: string): TgWebParams | null {
  try {
    const decodedParams = decode_base64(startParam);
    const decodedParamsStr = new TextDecoder().decode(decodedParams);

    const jsonParams: TgWebParams = JSON.parse(decodedParamsStr);

    return jsonParams;
  } catch (error) {
    console.error('Failed to parse Telegram start parameter:', error);
    return null;
  }
}

function getRedirectPath(params: TgWebParams): string {
  const command: TelegramWebCommand = params.command;

  if ('OpenSpacePage' in command) {
    const { space_id } = command.OpenSpacePage;
    return route.space(space_id);
  }

  return route.home();
}

function requestTelegramToken(telegramRaw: string) {
  return apiFetch<{ token: string }>(
    `${config.api_url}${ratelApi.telegram.verifyTelegramRaw()}`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        telegram_raw: telegramRaw,
      }),
    },
  );
}

function TelegramMiniAppMain() {
  const [isLoading, setIsLoading] = useState(true);
  const raw = useRawInitData();
  const router = useRouter();
  const searchParams = useSearchParams();
  const { ed25519KeyPair } = useAuth();
  const [token, setToken] = useState<string | null>(null);
  useEffect(() => {
    const tryLoginWithTelegramRaw = async () => {
      if (!ed25519KeyPair || !raw) {
        return;
      }
      try {
        const url = proxy.login.loginWithTelegram(raw);
        const info = await send(ed25519KeyPair, url, '');
        //If telegram User is not linked with Ratel Service, Open External Browser to Linking
        if (!info) {
          const res = await requestTelegramToken(raw);
          if (!res || !res.data) {
            throw new Error('Failed to get telegram token');
          }
          setToken(res.data.token);
          setIsLoading(false);
          return;
        }
        const tgWebAppStartParam = searchParams.get('tgWebAppStartParam');
        if (tgWebAppStartParam) {
          const params = parseTelegramStartParam(tgWebAppStartParam);
          if (params) {
            const redirectPath = getRedirectPath(params);
            router.replace(redirectPath);
          }
        } else {
          //telegram Webview close command
          postEvent('web_app_close');
        }
      } catch (error) {
        console.error('Error occurred while logging in:', error);
        setIsLoading(false);
      }
    };

    tryLoginWithTelegramRaw();
  }, [raw, ed25519KeyPair, searchParams, router]);

  return (
    <>
      {isLoading ? (
        <Loading />
      ) : (
        <div className="flex flex-col items-center justify-center w-full h-full">
          <Button
            variant="rounded_primary"
            onClick={() => {
              let url = `${window.location.origin}${route.login()}?service=Telegram&token=${token}`;
              let telegramDeepLink = `tg://resolve?domain=${config.telegram_botname}/app&startapp`;
              const tgWebAppStartParam = searchParams.get('tgWebAppStartParam');

              if (tgWebAppStartParam) {
                telegramDeepLink += `=${tgWebAppStartParam}`;
              }
              url += `&redirectUrl=${encodeURIComponent(telegramDeepLink)}`;

              postEvent('web_app_open_link', {
                url,
                try_instant_view: false,
              });
              postEvent('web_app_close');
            }}
          >
            Connect Telegram Account
          </Button>
        </div>
      )}
    </>
  );
}
