import { useEffect, useState } from 'react';
import { useRawInitData, postEvent, useLaunchParams } from '@tma.js/sdk-react';

import Loading from '../loading';
import { useNavigate, useSearchParams } from 'react-router';
import { decode_base64 } from '@/lib/base64';
import { route } from '@/route';

import { getKey as getUserQueryKey } from '../(social)/_hooks/user';
import { getQueryClient } from '@/providers/getQueryClient';
import { loginWithTelegram } from '@/lib/api/ratel/auth.v3';
import { SpaceType } from '@/features/spaces/types/space-type';

export interface TgWebParams {
  command: TelegramWebCommand;
}

export type TelegramWebCommand = {
  OpenSpacePage: {
    space_pk: string;
    type: SpaceType;
  };
};

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
    const { space_pk, type } = command.OpenSpacePage;
    return route.spaceByType(type, space_pk);
  }

  return route.home();
}

export default function TelegramPage() {
  const params = useLaunchParams();
  const tgWebAppStartParam = params.tgWebAppStartParam;
  const [isLoading, setIsLoading] = useState(true);
  const raw = useRawInitData();
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const queryClient = getQueryClient();
  console.log('TelegramPage initialized with params:', params, raw);
  useEffect(() => {
    const tryLoginWithTelegramRaw = async () => {
      try {
        const info = await loginWithTelegram(raw);

        if (!info) {
          setIsLoading(false);
          return;
        }

        queryClient.refetchQueries({ queryKey: getUserQueryKey() });

        if (tgWebAppStartParam) {
          const params = parseTelegramStartParam(tgWebAppStartParam);
          if (params) {
            const redirectPath = getRedirectPath(params);
            navigate(redirectPath);
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
  }, [raw, searchParams, navigate, queryClient, tgWebAppStartParam]);

  return (
    <>{isLoading ? <Loading /> : <div>Failed to login with telegram</div>}</>
  );
}
