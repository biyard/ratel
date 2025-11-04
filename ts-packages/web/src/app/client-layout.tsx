'use client';

import { useEffect, useState } from 'react';
import Header from '@/components/header';
import { route } from '@/route';
import { useAuth } from '@/lib/contexts/auth-context';
import { usePopup } from '@/lib/contexts/popup-service';
import { LoginModal } from '@/components/popup/login-popup';
import { useTranslation } from 'react-i18next';
import { NavLink } from 'react-router';
import { useUserInfo } from '@/hooks/use-user-info';
import { Col } from '@/components/ui/col';
import { SafeArea } from '@/components/ui/safe-area';
import Footer from '@/components/footer';

export default function ClientLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const { t } = useTranslation('Nav');
  const popup = usePopup();
  const { data, refetch, isLoading } = useUserInfo();
  const { logout } = useAuth();
  const [mobileExtends, setMobileExtends] = useState(false);

  useEffect(() => {
    document.body.style.overflow = mobileExtends ? 'hidden' : '';
    return () => {
      document.body.style.overflow = '';
    };
  }, [mobileExtends]);

  const linkClass =
    'font-bold text-menu-text text-[20px] hover:text-menu-text/80 flex flex-row w-full justify-center items-center';

  return (
    <>
      <Header
        mobileExtends={mobileExtends}
        setMobileExtends={setMobileExtends}
      />
      <div className="w-full min-h-screen">{children}</div>
      <Footer />
      <div
        className={
          mobileExtends
            ? 'fixed top-[60px] left-0 w-screen h-screen z-20 text-white bg-bg hidden max-tablet:flex max-tablet:flex-col max-tablet:items-start max-tablet:justify-start pt-6 px-4 gap-[50px]'
            : 'hidden'
        }
      >
        <NavLink
          to={route.settings()}
          onClick={() => setMobileExtends(false)}
          className={linkClass}
        >
          {data?.nickname}
        </NavLink>
        <NavLink
          to={route.home()}
          onClick={() => setMobileExtends(false)}
          className={linkClass}
        >
          {t('home')}
        </NavLink>

        {!isLoading && data ? (
          <div
            className={linkClass + ' cursor-pointer'}
            onClick={() => {
              logout();
              refetch();
              setMobileExtends(false);
            }}
          >
            {t('logout')}
          </div>
        ) : (
          <button
            className={linkClass + ' cursor-pointer'}
            onClick={() => {
              popup
                .open(<LoginModal />)
                .withTitle(t('join_the_movement'))
                .withoutBackdropClose();
            }}
          >
            {t('signIn')}
          </button>
        )}
      </div>
    </>
  );
}
