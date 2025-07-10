'use client';

import { useLaunchParams } from '@telegram-apps/sdk-react';

export default function TelegramMiniAppMain() {
  const lp = useLaunchParams();

  return (
    <div>
      <Value title="tgWebAppPlatform" value={lp.tgWebAppPlatform} />
      <Value title="tgWebAppShowSettings" value={lp.tgWebAppShowSettings} />
      <Value title="tgWebAppVersion" value={lp.tgWebAppVersion} />
      <Value title="tgWebAppBotInline" value={lp.tgWebAppBotInline} />
      <Value title="tgWebAppStartParam" value={lp.tgWebAppStartParam} />
      <Value title="tgWebAppData" type="link" value="/init-data" />
      <Value title="tgWebAppThemeParams" type="link" value="/theme-params" />
    </div>
  );
}

function Value({
  title,
  value,
  type,
}: {
  title: string;
  value?: string | boolean;
  type?: 'link';
}) {
  return (
    <div className="flex flex-col gap-2">
      <span className="text-sm text-gray-500">{title}</span>
      <span className="text-sm text-gray-800">{value || 'N/A'}</span>
      <span>{type}</span>
    </div>
  );
}
