import { useParams } from 'react-router';
import DaoPage from './dao-page';

export default function Page() {
  const { username } = useParams<{ username: string }>();

  return <DaoPage username={username!} />;
}
