import { useState } from 'react';
import { State } from '@/types/state';
import { useSuspenseUserInfo } from '@/hooks/use-user-info';

export class Controller {
  constructor(
    public state: State<boolean>,
    public did: string,
  ) {}
}

export function useController() {
  const state = useState(false);
  const { data: user } = useSuspenseUserInfo();

  // Generate DID from user PK (use actual DID when available)
  const did = `did:ratel:${user.pk.toLowerCase()}`;

  return new Controller(new State(state), did);
}
