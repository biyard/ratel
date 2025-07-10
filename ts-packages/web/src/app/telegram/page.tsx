'use client';

import { useEffect, useState } from 'react';
import TelegramMiniAppMain from './_components/params';

export function useDidMount(): boolean {
  const [didMount, setDidMount] = useState<boolean>(false);

  useEffect(() => {
    setDidMount(true);
  }, []);

  return didMount;
}

export default function HomePage() {
  const didMount = useDidMount();
  return (
    <div>
      <h2>My App</h2>
      {didMount ? <TelegramMiniAppMain /> : <p>Loading...</p>}
    </div>
  );
}
