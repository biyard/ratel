'use client';

import { useEffect, useState } from 'react';
import Header from '@/components/header';
import { route } from '@/route';
import { usePopup } from '@/lib/contexts/popup-service';
import { LoginModal } from '@/components/popup/login-popup';
import { useTranslation } from 'react-i18next';
import { NavLink, useMatches } from 'react-router';
import { useUserInfo } from '@/hooks/use-user-info';
import Footer from '@/components/footer';
import MobileSideMenu from '@/components/mobile-side-menu';
import { usePageTracking } from '@/features/analytics/hooks/use-analytics';
import {
  setAnalyticsUserId,
  setAnalyticsUserProperties,
} from '@/lib/service/analytics-service';

export default function ClientLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const { t } = useTranslation('Nav');
  const popup = usePopup();
  const { data } = useUserInfo();
  const [mobileExtends, setMobileExtends] = useState(false);
  const matches = useMatches();

  // Check if current route should hide header
  const hideHeader = matches.some(
    (match) =>
      match.handle && (match.handle as { hideHeader?: boolean }).hideHeader,
  );

  // Track page views automatically
  usePageTracking();

  // Track user identification for analytics
  useEffect(() => {
    if (data) {
      setAnalyticsUserId(data.pk);
      setAnalyticsUserProperties({
        username: data.username,
        email: data.email,
        nickname: data.nickname,
      });
    } else {
      setAnalyticsUserId(null);
    }
  }, [data]);

  useEffect(() => {
    document.body.style.overflow = mobileExtends ? 'hidden' : '';
    return () => {
      document.body.style.overflow = '';
    };
  }, [mobileExtends]);

  return (
    <>
      {!hideHeader && (
        <Header
          mobileExtends={mobileExtends}
          setMobileExtends={setMobileExtends}
        />
      )}
      <div
        className={
          hideHeader
            ? 'w-full min-h-screen'
            : 'w-full min-h-[calc(100vh-var(--header-height))]'
        }
      >
        {children}
      </div>
      <Footer />

      {/* Mobile Side Menu for authenticated users */}
      {!hideHeader && data && (
        <MobileSideMenu
          isOpen={mobileExtends}
          onClose={() => setMobileExtends(false)}
        />
      )}

      {/* Mobile Menu for non-authenticated users */}
      {!hideHeader && !data && (
        <div
          className={
            mobileExtends
              ? 'fixed top-(--header-height) left-0 w-screen h-[calc(100vh-var(--header-height))] z-50 bg-bg hidden max-tablet:flex max-tablet:flex-col max-tablet:items-start max-tablet:justify-start pt-6 px-4 gap-6'
              : 'hidden'
          }
        >
          <NavLink
            to={route.home()}
            onClick={() => setMobileExtends(false)}
            className="font-bold text-menu-text text-[20px] hover:text-menu-text/80 flex flex-row w-full justify-center items-center"
          >
            {t('home')}
          </NavLink>

          <button
            className="font-bold text-menu-text text-[20px] hover:text-menu-text/80 flex flex-row w-full justify-center items-center cursor-pointer"
            onClick={() => {
              popup
                .open(<LoginModal />)
                .withTitle(t('join_the_movement'))
                .withoutBackdropClose();
              setMobileExtends(false);
            }}
          >
            {t('signIn')}
          </button>
        </div>
      )}
    </>
  );
}
