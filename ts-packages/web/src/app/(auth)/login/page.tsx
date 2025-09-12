import { redirect } from 'next/navigation';
import { route } from '@/route';
import { isLoggedIn } from '@/lib/auth';
import Client from './client';

export default async function LoginPage({
  searchParams,
}: {
  searchParams?: Promise<{ [key: string]: string | string[] | undefined }>;
}) {
  const isLogin = await isLoggedIn();
  const paramsObj = searchParams ? await searchParams : {};

  if (isLogin) {
    let url;
    const currentParams = new URLSearchParams();

    for (const [key, value] of Object.entries(paramsObj)) {
      if (typeof value === 'string') {
        currentParams.set(key, value);
      }
    }

    if (currentParams.has('service')) {
      url = route.connect();
      const paramsString = currentParams.toString();
      if (paramsString) {
        url += `?${paramsString}`;
      }
    } else {
      url = route.home();
    }

    redirect(url);
  }

  return <Client />;
}
