import { isLoggedIn } from '@/lib/auth';
import { route } from '@/route';
import { redirect } from 'next/navigation';
import Client from './client';
import { Service } from '../store';

export default async function ConnectPage({
  searchParams,
}: {
  searchParams?: Promise<{ [key: string]: string | string[] | undefined }>;
}) {
  const isLogin = await isLoggedIn();
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

  const redirectUrl =
    typeof paramsObj.redirectUrl === 'string'
      ? paramsObj.redirectUrl
      : undefined;

  const serviceParam =
    typeof paramsObj.service === 'string'
      ? (paramsObj.service as Service)
      : undefined;

  const token =
    typeof paramsObj.token === 'string' ? paramsObj.token : undefined;

  return (
    <Client redirectUrl={redirectUrl} service={serviceParam} token={token} />
  );
}
