import { useState } from 'react';
import { State } from '@/types/state';
import { useSuspenseUserInfo } from '@/hooks/use-user-info';
import { useSuspenseDidData } from '../../hooks/use-did-data';
import { Age, Gender } from '@/components/icons';
import { useCredentialsI18n } from './i18n';
import { logger } from '@/lib/logger';
import { useIdentityVerification } from '../../hooks/use-identity-verification';

export class Controller {
  constructor(
    public state: State<Array<{ type: string; name: string; value: string }>>,
    public did: string,
    public t: ReturnType<typeof useCredentialsI18n>,
    public identify: ReturnType<typeof useIdentityVerification>,
  ) {}

  // FIXME: reflect language changes
  get attributes() {
    return this.state.get().map(({ type, name, value }) => {
      let Icon = Age;

      if (type === 'gender') {
        Icon = Gender;
      }

      return {
        Icon,
        attribute_name: name,
        attribute_value: value,
      };
    });
  }

  handleVerify = async () => {
    logger.debug('Veirfy credentials for DID:', this.did);
    const customer = await this.identify.mutateAsync();
    logger.debug('Verified customer info:', customer);

    const nowYear = new Date().getFullYear();

    this.state.set([
      {
        type: 'age',
        name: this.t.age,
        value: `${nowYear - Number(customer.birthDate.slice(0, 4))}`,
      },
      {
        type: 'gender',
        name: this.t.gender,
        value: this.t[customer.gender.toLowerCase()],
      },
    ]);
  };
}

export function useController() {
  const state = useState([]);
  const { data: user } = useSuspenseUserInfo();
  const { data: didDocument } = useSuspenseDidData();
  const t = useCredentialsI18n();
  const identify = useIdentityVerification();

  // Use DID from API, fallback to generated DID if not available
  const did = didDocument?.id ?? `did:web:ratel:${user.username}`;

  return new Controller(new State(state), did, t, identify);
}
