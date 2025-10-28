import { usePopup } from '@/lib/contexts/popup-service';
import { useTranslation } from 'react-i18next';
import { useApiCall } from '@/lib/api/use-send';
import { ratelApi } from '@/lib/api/ratel_api';
import { ChangeThemeRequest } from '@/lib/api/models/themes/theme';
import { useSuspenseUserInfo } from '@/hooks/use-user-info';
import { useQueryClient } from '@tanstack/react-query';
import { QK_USERS_GET_INFO } from '@/constants';
import LocaleModal from '../../settings/_components/modal/locale-modal';
import ThemeModal from '../../settings/_components/modal/theme-modal';
import SpecBox from '../../_components/spec-box';
import { useNavigate } from 'react-router';
import { ThemeType } from '@/lib/api/ratel/users.v3';

export default function ExperimentalSettingsPage() {
  const { post } = useApiCall();
  const { data } = useSuspenseUserInfo();
  const { t, i18n } = useTranslation('Settings');
  const popup = usePopup();
  const navigate = useNavigate();
  const locale = i18n.language as 'en' | 'ko';
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
            navigate(-1);
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
          {
            <SpecBox
              left_text="Theme"
              action_text={currentThemeLabel}
              onClick={handleChangeTheme}
            />
          }
        </div>
      </section>
    </div>
  );
}
