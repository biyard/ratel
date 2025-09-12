import { redirect } from 'next/navigation';
import { route } from '@/route';
import { isLoggedIn } from '@/lib/auth';
import Client from './client';

export default async function LoginPage() {
  const isLogin = await isLoggedIn();

  if (isLogin) {
    redirect(route.home());
  }

  return <Client />;
}
