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
import { getServerQueryClient } from '@/lib/query-utils.server';
import ReferralHandler from './_providers/referral-handler';

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
        {/* Initial theme script to avoid FOUC */}
        <script
          dangerouslySetInnerHTML={{
            __html:
              "(function(){try{var k='ratel.theme';var s=localStorage.getItem(k)||'system';var m=window.matchMedia&&window.matchMedia('(prefers-color-scheme: dark)').matches;var t=s==='system'?(m?'dark':'light'):s;if(t==='light'){document.documentElement.setAttribute('data-theme','light');}else{document.documentElement.removeAttribute('data-theme');}}catch(e){}})();",
          }}
        />
      </head>
      <body className={`${raleway.variable} antialiased bg-bg`}>
        <CookieProvider>
          <Providers dehydratedState={dehydratedState}>
            <HydrationBoundary state={dehydratedState}>
              <ReferralHandler />
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
