import { useMemo, useState } from 'react';
import { useSuspenseUserInfo } from '@/hooks/use-user-info';
import { useSuspenseDidData } from '../../hooks/use-did-data';
import { Age, Gender } from '@/components/icons';
import { useCredentialsI18n } from './i18n';
import { logger } from '@/lib/logger';
import { usePortOneAttributes } from '../../hooks/use-portone-attributes';
import { useVerifiedAttributes } from '../../hooks/use-verified-attributes';
import { useCodeVerification } from '../../hooks/use-code-verification';
import { University } from 'lucide-react';

export class Controller {
  constructor(
    public did: string,
    public t: ReturnType<typeof useCredentialsI18n>,
    public identify: ReturnType<typeof usePortOneAttributes>,
    public codeVerify: ReturnType<typeof useCodeVerification>,
    public attributes: Array<{
      Icon: React.FunctionComponent<React.ComponentProps<'svg'>>;
      attribute_name: string;
      attribute_value: string;
    }>,
    public isMethodModalOpen: boolean,
    public isCodeModalOpen: boolean,
    public openMethodModal: () => void,
    public closeMethodModal: () => void,
    public openCodeModal: () => void,
    public closeCodeModal: () => void,
  ) {}

  handleVerify = () => {
    logger.debug('Opening verification method modal for DID:', this.did);
    this.openMethodModal();
  };

  handleIdentityVerify = async () => {
    this.closeMethodModal();
    logger.debug('Verify credentials via PortOne for DID:', this.did);
    const customer = await this.identify.mutateAsync();
    logger.debug('Verified customer info:', customer);
  };

  handleCodeVerify = () => {
    this.closeMethodModal();
    this.openCodeModal();
  };

  handleCodeSubmit = async (code: string) => {
    logger.debug('Verify credentials via code:', code);
    await this.codeVerify.mutateAsync(code);
    this.closeCodeModal();
  };
}

export function useController() {
  const { data: user } = useSuspenseUserInfo();
  const { data: didDocument } = useSuspenseDidData();
  const t = useCredentialsI18n();
  const identify = usePortOneAttributes();
  const codeVerify = useCodeVerification();
  const { data: attributes } = useVerifiedAttributes();

  const [isMethodModalOpen, setIsMethodModalOpen] = useState(false);
  const [isCodeModalOpen, setIsCodeModalOpen] = useState(false);

  const attrs = useMemo(() => {
    logger.debug('User attributes:', attributes);
    const attrs = [];
    if (attributes.age) {
      attrs.push({
        Icon: Age,
        attribute_name: t.age,
        attribute_value: attributes.age,
      });
    }
    if (attributes.gender) {
      attrs.push({
        Icon: Gender,
        attribute_name: t.gender,
        attribute_value: t[attributes.gender.toLowerCase()],
      });
    }

    if (attributes.university) {
      attrs.push({
        Icon: University,
        attribute_name: t.university,
        attribute_value: attributes.university,
      });
    }

    return attrs;
  }, [attributes, t]);

  // Use DID from API, fallback to generated DID if not available
  const did = didDocument?.id ?? `did:web:ratel.foundation:${user.username}`;

  return new Controller(
    did,
    t,
    identify,
    codeVerify,
    attrs,
    isMethodModalOpen,
    isCodeModalOpen,
    () => setIsMethodModalOpen(true),
    () => setIsMethodModalOpen(false),
    () => setIsCodeModalOpen(true),
    () => setIsCodeModalOpen(false),
  );
}
