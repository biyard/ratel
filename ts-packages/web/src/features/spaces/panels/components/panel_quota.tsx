import { Input } from '@/components/ui/input';
import { useEffect, useRef, useState } from 'react';

export type PanelQuotasProps = {
  quotas: number;
  canEdit: boolean;
  setQuotas: (quotas: number) => void;
};

export function PanelQuotas({ quotas, canEdit, setQuotas }: PanelQuotasProps) {
  const [editMode, setEditMode] = useState(false);
  const [internalQuota, setInternalQuota] = useState('');
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (!editMode) setInternalQuota(String(quotas ?? 0));
  }, [quotas, editMode]);

  useEffect(() => {
    if (editMode) inputRef.current?.focus();
  }, [editMode]);

  const commit = () => {
    const n = internalQuota.trim() === '' ? 0 : Number(internalQuota.trim());
    setQuotas(Number.isFinite(n) ? n : 0);
    setEditMode(false);
  };

  const cancel = () => {
    setInternalQuota(String(quotas ?? 0));
    setEditMode(false);
  };

  const onKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      commit();
    }
    if (e.key === 'Escape') {
      e.preventDefault();
      cancel();
    }
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
    if (allowed.includes(e.key) || isDigit || e.metaKey || e.ctrlKey) return;
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
          className="w-[80px] text-center font-bold"
          value={internalQuota}
          onKeyDown={(e) => {
            onBeforeKeyDown(e);
            onKeyDown(e);
          }}
          onChange={(e) => setInternalQuota(sanitize(e.target.value))}
          onBlur={commit}
          placeholder="0"
        />
      ) : (
        <div
          role="button"
          tabIndex={0}
          className="w-[80px] h-9 inline-flex items-center justify-center rounded-md border border-transparent font-medium text-sm text-text-primary cursor-text"
          onClick={() => {
            if (!canEdit) return;
            setInternalQuota(String(quotas ?? 0));
            setEditMode(true);
          }}
          onKeyDown={(e) => {
            if (e.key === 'Enter' && canEdit) {
              setInternalQuota(String(quotas ?? 0));
              setEditMode(true);
            }
          }}
        >
          {quotas ?? 0}
        </div>
      )}
    </div>
  );
}
