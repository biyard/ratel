import { Input } from '@/components/ui/input';
import { executeOnKeyStroke } from '@/utils/key-event-handle';
import { useEffect, useRef, useState } from 'react';

export type PanelQuotasProps = {
  quotas: number;
  canEdit: boolean;
  setQuotas: (quotas: number) => void;
};

export function PanelQuotas({ quotas, canEdit, setQuotas }: PanelQuotasProps) {
  const [editMode, setEditMode] = useState(false);
  const [internalQuota, setInternalQuota] = useState(String(quotas ?? 0));
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (!editMode) setInternalQuota(String(quotas ?? 0));
  }, [quotas, editMode]);

  useEffect(() => {
    if (editMode) inputRef.current?.focus();
  }, [editMode]);

  const commit = () => {
    const v = internalQuota.trim();
    const n = v === '' ? 0 : Number(v);
    setQuotas(Number.isFinite(n) ? n : 0);
    setEditMode(false);
  };

  const cancel = () => {
    setInternalQuota(String(quotas ?? 0));
    setEditMode(false);
  };

  const onKeyDown = (e: React.KeyboardEvent) => {
    executeOnKeyStroke(e, commit, cancel);
  };

  const onBeforeKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    const allowed = [
      'Backspace',
      'Delete',
      'ArrowLeft',
      'ArrowRight',
      'Tab',
      'Home',
      'End',
      'Enter',
    ];
    const isDigit = e.key >= '0' && e.key <= '9';
    if (allowed.includes(e.key) || isDigit) return;
    if (e.metaKey || e.ctrlKey) return;
    e.preventDefault();
  };

  const sanitize = (s: string) => s.replace(/\D+/g, '');

  return (
    <div>
      {editMode && canEdit ? (
        <Input
          ref={inputRef}
          type="text"
          inputMode="numeric"
          pattern="[0-9]*"
          className="w-full border-b border-transparent !border-b-white focus:!border-transparent focus:rounded-md font-bold text-text-primary text-[14px]/[30px] placeholder:text-neutral-300 placeholder:font-medium rounded-none"
          value={internalQuota}
          onKeyDown={(e) => {
            onBeforeKeyDown(e);
            onKeyDown(e);
          }}
          onChange={(e) => setInternalQuota(sanitize(e.target.value))}
          onPaste={(e) => {
            e.preventDefault();
            const text =
              // eslint-disable-next-line @typescript-eslint/no-explicit-any
              (e.clipboardData || (window as any).clipboardData).getData(
                'text',
              );
            setInternalQuota(sanitize(text));
          }}
          onBlur={commit}
          placeholder="0"
        />
      ) : (
        <div
          role="button"
          tabIndex={0}
          className="flex w-full items-center min-h-[30px] cursor-text"
          onClick={() => setEditMode(true)}
          onKeyDown={(e) => e.key === 'Enter' && setEditMode(true)}
        >
          <div className="font-bold text-text-primary text-[14px]/[30px]">
            {quotas ?? 0}
          </div>
        </div>
      )}
    </div>
  );
}
