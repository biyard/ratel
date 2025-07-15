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

function TelegramMiniAppMain() {
  const [isLoading, setIsLoading] = useState(true);
  const raw = useRawInitData();
  const popup = usePopup();
  const router = useRouter();
  const searchParams = useSearchParams();
  const { setTelegramRaw } = useAuth();
  const anonKeyPair = useEd25519KeyPair();
  const { refetch } = useUserInfo();
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
          const decodedParams = decode_base64(tgWebAppStartParam);
          const decodedParamsStr = new TextDecoder().decode(decodedParams);
          const jsonParams = JSON.parse(decodedParamsStr);
          router.replace(`${route.telegramSprintLeague(jsonParams.space_id)}`);
        }
      } catch (error) {
        console.error('Error occurred while logging in:', error);
        setIsLoading(false);
        // FIXME: When Server is not available, this popup cause infinite loop
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
  }, [raw]);

  return <>{isLoading ? <Loading /> : <></>}</>;
}
