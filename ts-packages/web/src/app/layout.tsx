import { Raleway } from 'next/font/google';
import '@/assets/css/globals.css';
import Providers from '@/providers/providers';
import CookieProvider from './_providers/CookieProvider';
import { PopupZone } from '@/components/popupzone';
import ClientLayout from './(social)/_components/client-layout';
import { dehydrate, HydrationBoundary } from '@tanstack/react-query';
import { ToastContainer } from 'react-toastify';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import 'react-toastify/dist/ReactToastify.css';
import { prefetchUserInfo } from './(social)/_hooks/user';
import Script from 'next/script';
import { getServerQueryClient } from '@/lib/query-utils.server';

const raleway = Raleway({
  variable: '--font-raleway',
  weight: ['100', '200', '300', '400', '500', '600', '700', '800', '900'],
  subsets: ['latin'],
});

export default async function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const queryClient = await getServerQueryClient();

  await prefetchUserInfo(queryClient);
  const dehydratedState = dehydrate(queryClient);

  return (
    <html lang="en">
      <head>
        <link rel="icon" href="/logos/favicon.ico" />
        <Script src="https://telegram.org/js/telegram-web-app.js?57" />
      </head>
      <body className={`${raleway.variable} antialiased bg-bg`}>
        <CookieProvider>
          <Providers dehydratedState={dehydratedState}>
            <HydrationBoundary state={dehydratedState}>
              <ClientLayout>{children}</ClientLayout>
              <PopupZone />
            </HydrationBoundary>
            <ReactQueryDevtools initialIsOpen={false} />
          </Providers>
        </CookieProvider>
        <ToastContainer />
      </body>
    </html>
  );
}
