import Providers from '@/providers/providers';
import { PopupZone } from '@/components/popupzone';
import ClientLayout from './(social)/_components/client-layout';
import { ToastContainer } from 'react-toastify';
import { ReactQueryDevtools } from '@tanstack/react-query-devtools';
import 'react-toastify/dist/ReactToastify.css';
import { Outlet } from 'react-router';

export default function RootLayout() {
  return (
    <div className={`antialiased bg-bg`}>
      <Providers>
        <ClientLayout>
          <Outlet />
        </ClientLayout>
        <PopupZone />
        <ReactQueryDevtools initialIsOpen={false} />
      </Providers>
      <ToastContainer />
    </div>
  );
}
