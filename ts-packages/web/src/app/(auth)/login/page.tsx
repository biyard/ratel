import { route } from '@/route';
import Client from './client';
import { useLoaderData, useNavigate } from 'react-router';
import { useUserInfo } from '@/hooks/use-user-info';

export default function LoginPage() {
  const paramsObj = useLoaderData();
  const nav = useNavigate();
  const { data: user } = useUserInfo();
  const isLogin = user !== null;

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

    nav(url, { replace: true });
  }

  return <Client />;
}
