import { route } from '@/route';
import Client from './client';
import { useUserInfo } from '@/hooks/use-user-info';
import { useNavigate } from 'react-router';

export default async function LoginPage() {
  const { data: user } = useUserInfo();
  const nav = useNavigate();
  const isLogin = user !== null;

  if (isLogin) {
    nav(route.home(), { replace: true });
  }

  return <Client />;
}
