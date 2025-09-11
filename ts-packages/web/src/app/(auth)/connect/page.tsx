import { isLoggedIn } from '@/lib/auth';
import { route } from '@/route';
import { redirect } from 'next/navigation';
import Client from './client';

export default async function ConnectPage({
  searchParams,
}: {
  searchParams?: Promise<{ [key: string]: string | string[] | undefined }>;
}) {
  const isLogin = await isLoggedIn();
  console.log('connect page isLogin', isLogin);
  const paramsObj = searchParams ? await searchParams : {};
  if (!isLogin) {
    let url;
    const currentParams = new URLSearchParams();

    for (const [key, value] of Object.entries(paramsObj)) {
      if (typeof value === 'string') {
        currentParams.set(key, value);
      }
    }

    url = route.login();
    const paramsString = currentParams.toString();
    if (paramsString) {
      url += `?${paramsString}`;
    }

    redirect(url);
  }

  return <Client />;
}
