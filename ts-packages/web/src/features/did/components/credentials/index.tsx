import { useState } from 'react';
import { Col } from '@/components/ui/col';
import { Row } from '@/components/ui/row';
import { useController } from './use-controller';
import { Verified } from '@/components/icons';
import { useCredentialsI18n } from './i18n';
import Heading from '@/components/ui/heading';
import { Button } from '@/components/ui/button';
import Card from '@/components/card';
import VerifiedItem from './verified_item';

export function Credentials() {
  const ctrl = useController();
  const t = useCredentialsI18n();

  return (
    <>
      <Col className="gap-4">
        {/* Verifiable Credential Card */}
        <Col
          mainAxisAlignment="center"
          crossAxisAlignment="center"
          className="overflow-hidden relative py-6 gap-[17.5px]"
          rounded="default"
          padding="sm"
          style={{
            background:
              'radial-gradient(circle at center, rgba(77, 92, 255, 0.5) 0%, rgba(30, 30, 30, 1) 100%)',
          }}
        >
          <Verified className="w-20 h-20" />
          <Col
            mainAxisAlignment="center"
            crossAxisAlignment="center"
            className="gap-1"
          >
            <Heading variant="heading4">{t.vc}</Heading>
            <p className="text-sm text-neutral-300 light:text-neutral-700 dark:text-neutral-300">
              {t.id}: {ctrl.did}
            </p>
          </Col>
        </Col>

        {/* My DID Section Header */}
        <Col rounded="default" padding="sm" className="gap-5 bg-component-bg">
          <Heading variant="heading6">{t.my_did}</Heading>

          {ctrl.attributes.map((attr) => (
            <VerifiedItem {...attr} />
          ))}
          {ctrl.attributes.length === 0 && (
            <Card
              variant="outlined"
              className="flex items-center text-text-primary"
            >
              {t.no_data}
            </Card>
          )}

          <Row mainAxisAlignment="center" className="">
            <Button
              variant="text"
              className="text-primary hover:text-primary/60"
              onClick={ctrl.handleVerify}
              data-testid="credential-verify-button"
            >
              {t.verify}
            </Button>
          </Row>
        </Col>
      </Col>

      {/* Verification Method Selection Modal */}
      {ctrl.isMethodModalOpen && (
        <VerificationMethodModal
          onIdentityVerify={ctrl.handleIdentityVerify}
          onCodeVerify={ctrl.handleCodeVerify}
          onClose={ctrl.closeMethodModal}
          t={t}
        />
      )}

      {/* Code Input Modal */}
      {ctrl.isCodeModalOpen && (
        <CodeInputModal
          onSubmit={ctrl.handleCodeSubmit}
          onClose={ctrl.closeCodeModal}
          isSubmitting={ctrl.codeVerify.isPending}
          t={t}
        />
      )}
    </>
  );
}

function VerificationMethodModal({
  onIdentityVerify,
  onCodeVerify,
  onClose,
  t,
}: {
  onIdentityVerify: () => void;
  onCodeVerify: () => void;
  onClose: () => void;
  t: ReturnType<typeof useCredentialsI18n>;
}) {
  return (
    <div className="flex fixed inset-0 z-50 justify-center items-center bg-black bg-opacity-50">
      <div className="p-6 w-full max-w-md bg-white rounded-lg shadow-lg dark:bg-gray-800">
        <h2 className="mb-6 text-xl font-bold text-modal-label-text">
          {t.selectVerificationMethod}
        </h2>

        <div className="flex flex-col gap-4">
          {/* Identity Verification Option */}
          <button
            onClick={onIdentityVerify}
            className="p-4 text-left rounded-lg border border-gray-300 transition-all dark:border-gray-600 hover:bg-blue-50 hover:border-blue-500 dark:hover:border-blue-500 dark:hover:bg-gray-700"
            data-testid="identity-verification-option"
          >
            <h3 className="mb-1 text-lg font-semibold text-gray-700 dark:text-gray-500">
              {t.identityVerification}
            </h3>
            <p className="text-sm text-gray-600 dark:text-gray-400">
              {t.identityVerificationDesc}
            </p>
          </button>

          {/* Code Verification Option */}
          <button
            onClick={onCodeVerify}
            className="p-4 text-left rounded-lg border border-gray-300 transition-all dark:border-gray-600 hover:bg-blue-50 hover:border-blue-500 dark:hover:border-blue-500 dark:hover:bg-gray-700"
            data-testid="code-verification-option"
          >
            <h3 className="mb-1 text-lg font-semibold text-gray-700 dark:text-gray-500">
              {t.codeVerification}
            </h3>
            <p className="text-sm text-gray-600 dark:text-gray-400">
              {t.codeVerificationDesc}
            </p>
          </button>
        </div>

        <div className="flex justify-end mt-6">
          <Button
            className="hover:text-white text-neutral-500"
            onClick={onClose}
          >
            {t.cancel}
          </Button>
        </div>
      </div>
    </div>
  );
}

function CodeInputModal({
  onSubmit,
  onClose,
  isSubmitting,
  t,
}: {
  onSubmit: (code: string) => Promise<void>;
  onClose: () => void;
  isSubmitting: boolean;
  t: ReturnType<typeof useCredentialsI18n>;
}) {
  const [code, setCode] = useState('');
  const [error, setError] = useState('');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');

    if (!code.trim()) {
      setError(t.invalidCode);
      return;
    }

    try {
      await onSubmit(code.trim());
    } catch {
      setError(t.verificationError);
    }
  };

  return (
    <div className="flex fixed inset-0 z-50 justify-center items-center bg-black bg-opacity-50">
      <div className="p-6 w-full max-w-md bg-white rounded-lg shadow-lg dark:bg-gray-800">
        <h2 className="mb-4 text-xl font-bold text-modal-label-text">
          {t.enterCode}
        </h2>

        <form onSubmit={handleSubmit}>
          <div className="mb-4">
            <input
              type="text"
              value={code}
              onChange={(e) => setCode(e.target.value)}
              placeholder={t.codePlaceholder}
              className="py-2 px-3 w-full rounded border border-gray-300 dark:bg-gray-700 dark:border-gray-600 text-neutral-500"
              disabled={isSubmitting}
              autoFocus
              data-testid="credential-code-input"
            />
          </div>

          {error && <div className="mb-4 text-sm text-red-500">{error}</div>}

          <div className="flex gap-2 justify-end">
            <Button
              className="hover:text-white text-neutral-500"
              type="button"
              onClick={onClose}
              disabled={isSubmitting}
            >
              {t.cancel}
            </Button>
            <Button
              type="submit"
              variant="primary"
              disabled={isSubmitting || !code.trim()}
              data-testid="credential-code-submit"
            >
              {isSubmitting ? t.submitting : t.submit}
            </Button>
          </div>
        </form>
      </div>
    </div>
  );
}
