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

function useDidMount(): boolean {
  const [didMount, setDidMount] = useState<boolean>(false);

  useEffect(() => {
    setDidMount(true);
  }, []);

  return didMount;
}

export default function HomePage() {
  const didMount = useDidMount();

  return (
    <div className="absolute w-screen h-screen left-0 top-0 bg-bg">
      {didMount && <TelegramMiniAppMain />}
    </div>
  );
}

function TelegramMiniAppMain() {
  const [isLoading, setLoading] = useState(true);
  const router = useRouter();
  const searchParams = useSearchParams();
  const raw = useRawInitData();
  const popup = usePopup();
  const { setTelegramRaw } = useAuth();
  const anonKeyPair = useEd25519KeyPair();
  const { refetch } = useUserInfo();
  useEffect(() => {
    const loginWithTelegram = async (raw: string) => {
      console.debug('Telegram raw init data:', raw);

      const url = proxy.login.loginWithTelegram(raw);
      try {
        const info = await send(anonKeyPair, url, '');
        if (info) {
          console.log('Login successful:', info);
          refetch();
          const redirectUrl = searchParams.get('redirectURL');

          if (redirectUrl) {
            const newParams = new URLSearchParams(searchParams.toString());
            newParams.delete('redirectURL');
            const remainingParams = newParams.toString();
            const finalUrl = remainingParams
              ? `${redirectUrl}?${remainingParams}`
              : redirectUrl;

            router.replace(finalUrl);
            return;
          }
        } else {
          setLoading(false);
          popup
            .open(<LoginModal />)
            .withTitle('Join the Movement')
            .withoutClose()
            .withoutBackdropClose();
        }
      } catch (error) {
        console.error('Error occurred while logging in:', error);
      }
    };

    if (raw && popup && isLoading) {
      setTelegramRaw(raw);
      loginWithTelegram(raw);
    }
  }, [
    raw,
    setTelegramRaw,
    anonKeyPair,
    popup,
    isLoading,
    refetch,
    router,
    searchParams,
  ]);

  return <>{isLoading ? <Loading /> : <></>}</>;
}
