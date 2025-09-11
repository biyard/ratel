'use client';

import { config } from '@/config';
import ChevronRight from '@/assets/icons/chevron-right.svg';
import { usePopup } from '@/lib/contexts/popup-service';
import React from 'react';
import LocaleModal from '../modal/locale-modal';
import { useRouter } from 'next/navigation';
import { useLocale, useTranslations } from 'next-intl';
import ThemeModal from '../modal/theme-modal';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi } from '@/lib/api/ratel_api';
import { ChangeThemeRequest } from '@/lib/api/models/themes/theme';
import { useSuspenseUserInfo } from '@/lib/api/hooks/users';
import { ThemeType } from '@/lib/api/models/user';
import { useQueryClient } from '@tanstack/react-query';
import { QK_USERS_GET_INFO } from '@/constants';

export default function MySettings() {
  const { post } = useApiCall();
  const { data } = useSuspenseUserInfo();
  const t = useTranslations('Settings');
  const popup = usePopup();
  const router = useRouter();
  const locale = useLocale() as 'en' | 'ko';
  const qc = useQueryClient();

  const actionText = locale === 'ko' ? 'Korean' : 'English';
  const currentThemeLabel =
    data?.theme === ThemeType.Light
      ? 'Light'
      : data?.theme === ThemeType.Dark
        ? 'Dark'
        : 'System';

  const changeTheme = async (theme: 'light' | 'dark' | 'system') => {
    const value = theme === 'light' ? 1 : theme === 'dark' ? 2 : 3;
    await post(ratelApi.themes.changeTheme(), ChangeThemeRequest(value));
  };

  const handleChangeLanguage = () => {
    popup
      .open(
        <LocaleModal
          initialLocale={locale}
          onSave={(newLocale) => {
            document.cookie = `locale=${newLocale}; path=/; max-age=31536000; samesite=lax`;
            router.refresh();
            popup.close();
          }}
          onCancel={() => popup.close()}
        />,
      )
      .withTitle(t('select_language'));
  };

  const handleChangeTheme = () => {
    const initialTheme: 'light' | 'dark' | 'system' =
      data?.theme === ThemeType.Light
        ? 'light'
        : data?.theme === ThemeType.Dark
          ? 'dark'
          : 'system';
    const prevTheme = initialTheme;

    popup
      .open(
        <ThemeModal
          initialTheme={initialTheme}
          onPreview={async (newTheme) => {
            await changeTheme(newTheme);
            await qc.invalidateQueries({ queryKey: [QK_USERS_GET_INFO] });
          }}
          onSave={async (newTheme) => {
            await changeTheme(newTheme);
            await qc.invalidateQueries({ queryKey: [QK_USERS_GET_INFO] });
            popup.close();
          }}
          onCancel={() => {
            changeTheme(prevTheme)
              .then(() =>
                qc.invalidateQueries({ queryKey: [QK_USERS_GET_INFO] }),
              )
              .finally(() => popup.close());
          }}
        />,
      )
      .withTitle('Theme');
  };

  return (
    <div className="w-full max-w-[800px] mx-auto flex flex-col gap-6 px-4 md:px-0">
      <section className="bg-card-bg border border-card-border p-4 md:p-6 rounded-lg">
        <h2 className="text-lg font-bold mb-4 text-text-primary">
          {t('appearance')}
        </h2>
        <div className="flex flex-col gap-4">
          <SpecBox
            left_text={t('language')}
            action_text={actionText}
            onClick={handleChangeLanguage}
          />
          {config.env === 'local' && (
            <SpecBox
              left_text="Theme"
              action_text={currentThemeLabel}
              onClick={handleChangeTheme}
            />
          )}
        </div>
      </section>
    </div>
  );
}

function SpecBox({
  left_text,
  action_text,
  onClick,
}: {
  left_text: string;
  action_text?: string;
  onClick?: () => void;
}) {
  return (
    <div className="flex items-center justify-between border border-setting-card-border px-4 py-8 rounded-md">
      <p className="text-base font-bold text-text-primary">{left_text}</p>
      <button
        className="flex items-center gap-2 text-primary cursor-pointer"
        onClick={onClick}
      >
        {action_text}
        <ChevronRight className="w-4 h-4" />
      </button>
    </div>
  );
}
