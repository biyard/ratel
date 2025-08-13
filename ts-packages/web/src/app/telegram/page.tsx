'use client';

import { useEffect, useState } from 'react';
import { useRawInitData } from '@telegram-apps/sdk-react';
import { useAuth, useEd25519KeyPair } from '@/lib/contexts/auth-context';
import { proxy } from '@/lib/api/ratel_api';
import { send } from '@/lib/api/send';
import { LoginModal } from '@/components/popup/login-popup';
import { usePopup } from '@/lib/contexts/popup-service';
import { useUserInfo } from '../(social)/_hooks/user';
import Loading from '../loading';
import { useRouter, useSearchParams } from 'next/navigation';
import { decode_base64 } from '@/lib/base64';
import { route } from '@/route';
import { TgWebParams } from '@/types/telegram';
import { postEvent } from '@telegram-apps/sdk';

// FIXME: This Page is too complex, consider refactoring.

/*
  AS-IS
  1. When Telegram Web App Started, it tries to login with "telegram_raw"(telegram login info).
  2-A. If login with "telegram_raw" is successful
    a. it checks for "tgWebAppStartParam" in the URL.
    b. If "tgWebAppStartParam" exists, it parses the command and redirects to the appropriate page.
    c. If no "tgWebAppStartParam", it closes the Web App.
  2-B. If login fails, it opens a login popup.
    a. The Login popup allows the user connect Telegram to there account, or sign up with telegram info.
    b. After successful login(sign-up), it refetches `userinfo`.
    c. then go boack to step 1. (because of `useEffect` dependency on `data`)

*/

function useDidMount(): boolean {
  const [didMount, setDidMount] = useState<boolean>(false);

  useEffect(() => {
    setDidMount(true);
  }, []);

  return didMount;
}

export default function HomePage() {
  const didMount = useDidMount();

  return <>{didMount && <TelegramMiniAppMain />}</>;
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
  const command = params.command;

  if ('Subscribe' in command) {
    const { chat_id, lang } = command.Subscribe;
    return route.telegramSubscribe(chat_id, lang);
  }

  if ('SprintLeague' in command) {
    const { space_id } = command.SprintLeague;
    return route.telegramSprintLeague(space_id);
  }

  return route.home();
}

function TelegramMiniAppMain() {
  const [isLoading, setIsLoading] = useState(true);
  const [loginPopupOpened, setLoginPopupOpened] = useState(false);
  const raw = useRawInitData();
  const popup = usePopup();
  const router = useRouter();
  const searchParams = useSearchParams();
  const { setTelegramRaw } = useAuth();
  const anonKeyPair = useEd25519KeyPair();
  const { data, refetch } = useUserInfo();

  useEffect(() => {
    if (!popup.popup && loginPopupOpened) {
      refetch();
    }
  }, [loginPopupOpened, popup, refetch]);
  useEffect(() => {
    const loginWithTelegram = async (raw: string) => {
      setTelegramRaw(raw);
      const url = proxy.login.loginWithTelegram(raw);
      try {
        const info = await send(anonKeyPair, url, '');
        if (!info) {
          throw new Error('Login failed, no info returned');
        }
        refetch();

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
        // FIXME: When Server is not available, this popup cause infinite loop
        setLoginPopupOpened(true);
        popup
          .open(<LoginModal disableClose />)
          .withTitle('Join the Movement')
          .withoutClose()
          .withoutBackdropClose();
      }
    };

    if (!!raw) {
      loginWithTelegram(raw);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [raw, data]);

  return <>{isLoading ? <Loading /> : <></>}</>;
}
