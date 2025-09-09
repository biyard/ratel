'use client';

import { config } from '@/config';
import ChevronRight from '@/assets/icons/chevron-right.svg';
import { usePopup } from '@/lib/contexts/popup-service';
import React from 'react';
import LocaleModal from '../modal/locale-modal';
import { useRouter } from 'next/navigation';
import { useLocale, useTranslations } from 'next-intl';
import ThemeModal from '../modal/theme-modal';
import { useTheme } from 'next-themes';

export default function MySettings() {
  const t = useTranslations('Settings');
  const popup = usePopup();
  const router = useRouter();
  const locale = useLocale() as 'en' | 'ko';
  const { theme, setTheme } = useTheme();

  const actionText = locale === 'ko' ? 'Korean' : 'English';

  const currentThemeLabel =
    theme === 'light' ? 'Light' : theme === 'dark' ? 'Dark' : 'Dark';

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
    popup
      .open(
        <ThemeModal
          initialTheme={(theme as 'light' | 'dark' | 'system') ?? 'system'}
          onSave={(newTheme) => {
            setTheme(newTheme);
            popup.close();
          }}
          onCancel={() => popup.close()}
        />,
      )
      .withTitle('Theme');
  };

  return (
    <div className="w-full max-w-[800px] mx-auto flex flex-col gap-6 px-4 md:px-0">
      <section className="bg-component-bg p-4 md:p-6 rounded-lg">
        <h2 className="text-lg font-bold mb-4 text-foreground">
          {t('appearance')}
        </h2>

        <div className="flex flex-col gap-4">
          <SpecBox
            left_text={t('language')}
            action_text={actionText}
            onClick={handleChangeLanguage}
          />

          {/* theme box controller */}
          {config.env == 'dev' ? (
            <SpecBox
              left_text="Theme"
              action_text={currentThemeLabel}
              onClick={handleChangeTheme}
            />
          ) : (
            <></>
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
    <div className="flex items-center justify-between border border-neutral-800 px-4 py-8 rounded-md">
      <p className="text-lg font-bold text-sm text-foreground">{left_text}</p>

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
