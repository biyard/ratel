import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { TFunction } from 'i18next';
import { useState } from 'react';

export interface InviteCodeBoxProps {
  t: TFunction<'SpaceInvitationViewer', undefined>;
  verify: (code: string) => void;
}

export default function InviteCodeBox({ t, verify }: InviteCodeBoxProps) {
  const [code, setCode] = useState('');

  return (
    <div className="w-full flex items-center justify-center mb-5">
      <div className="w-full rounded-sm border border-card-border bg-card-bg p-6 dark:border-neutral-800 dark:bg-neutral-900">
        <div className="mb-1 text-xl font-semibold">
          {t('invitation_code_title')}
        </div>
        <div className="mb-6 text-sm text-neutral-500 dark:text-neutral-400">
          {t('invitation_code_desc')}
        </div>

        <label className="block text-sm font-medium mb-2">
          {t('verification_code')}
        </label>
        <Input
          autoFocus
          inputMode="text"
          spellCheck={false}
          value={code}
          onChange={(e) => {
            setCode(e.target.value);
          }}
          placeholder={t('verification_code_hint')}
        />

        <div className="mt-6 flex gap-2 w-full justify-end">
          <Button
            type="submit"
            variant="primary"
            className="w-fit"
            onClick={() => {
              verify(code);
            }}
          >
            {t('verify')}
          </Button>
        </div>
      </div>
    </div>
  );
}
