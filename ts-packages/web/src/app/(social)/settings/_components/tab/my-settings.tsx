'use client';

import { usePopup } from '@/lib/contexts/popup-service';
import LocaleModal from '../modal/locale-modal';
import { useTranslation } from 'react-i18next';
import ThemeModal from '../modal/theme-modal';
import SpecBox from '@/app/(social)/_components/spec-box';
import { useTheme } from '@/hooks/use-theme';

export default function MySettings() {
  const { t, i18n } = useTranslation('Settings');
  const popup = usePopup();
  const locale = i18n.language as 'en' | 'ko';
  const actionText = locale === 'ko' ? 'Korean' : 'English';
  const { theme, setTheme } = useTheme();
  const currentThemeLabel =
    theme === 'light' ? 'Light' : theme === 'dark' ? 'Dark' : 'System';

  const handleChangeLanguage = () => {
    popup
      .open(
        <LocaleModal
          initialLocale={locale}
          onSave={(newLocale) => {
            localStorage.setItem('user-language', newLocale);
            document.cookie = `locale=${newLocale}; path=/; max-age=31536000; samesite=lax`;
            i18n.changeLanguage(newLocale);
            popup.close();
          }}
          onCancel={() => popup.close()}
        />,
      )
      .withTitle(t('select_language'));
  };

  const handleChangeTheme = () => {
    const prevTheme = theme;
    popup
      .open(
        <ThemeModal
          initialTheme={theme}
          onPreview={(newTheme) => {
            setTheme(newTheme);
          }}
          onSave={(newTheme) => {
            setTheme(newTheme);
            popup.close();
          }}
          onCancel={() => {
            setTheme(prevTheme);
            popup.close();
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
            data-pw="language-setting-box"
          />
          {
            <SpecBox
              left_text="Theme"
              action_text={currentThemeLabel}
              onClick={handleChangeTheme}
              data-pw="theme-setting-box"
            />
          }
        </div>
      </section>
    </div>
  );
}
