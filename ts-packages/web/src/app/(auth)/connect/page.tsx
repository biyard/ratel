'use client';

import { route } from '@/route';
import Client from './client';
import { Service } from '../store';
import { useNavigate, useSearchParams } from 'react-router';
import { useCookie } from '@/lib/contexts/cookie-context';
import { useEffect } from 'react';

export default function ConnectPage() {
  const { isLoggedIn } = useCookie();
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();

  useEffect(() => {
    if (!isLoggedIn) {
      let url;
      const currentParams = new URLSearchParams(searchParams);
      url = route.login();
      const paramsString = currentParams.toString();
      if (paramsString) {
        url += `?${paramsString}`;
      }
      navigate(url, { replace: true });
    }
  }, [isLoggedIn, navigate, searchParams]);

  const redirectUrl = searchParams.get('redirectUrl') ?? undefined;
  const serviceParam = searchParams.get('service') as Service | undefined;
  const token = searchParams.get('token') ?? undefined;

  return (
    <Client redirectUrl={redirectUrl} service={serviceParam} token={token} />
  );
}
