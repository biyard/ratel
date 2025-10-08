import Providers from '@/providers/providers';
import { PopupZone } from '@/components/popupzone';
import ClientLayout from './(social)/_components/client-layout';
/* import { dehydrate, HydrationBoundary } from '@tanstack/react-query'; */
import { ToastContainer } from 'react-toastify';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import 'react-toastify/dist/ReactToastify.css';
/* import { prefetchUserInfo } from './(social)/_hooks/user';
 * import { getServerQueryClient } from '@/lib/query-utils.server'; */
/* import ReferralHandler from './_providers/referral-handler'; */
/* import { NextIntlClientProvider } from 'next-intl';
 * import { getLocale, getMessages } from 'next-intl/server';
 * import ThemeWrapper from './theme-wrapper'; */
import { Outlet } from 'react-router';

export default function RootLayout() {
  const locale = 'en';

  return (
    <div className={`antialiased bg-bg`}>
      {/* <NextIntlClientProvider locale={locale} messages={messages}> */}
      {/* <ThemeWrapper> */}
      {/* <ReferralHandler /> */}
      <Providers>
        <ClientLayout>
          <Outlet />
        </ClientLayout>
        <PopupZone />
        <ReactQueryDevtools initialIsOpen={false} />
        {/* </ThemeWrapper> */}
      </Providers>
      <ToastContainer />
      {/* </NextIntlClientProvider> */}
    </div>
  );
}
