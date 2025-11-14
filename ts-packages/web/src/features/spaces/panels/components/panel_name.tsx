import { Input } from '@/components/ui/input';
import { executeOnKeyStroke } from '@/utils/key-event-handle';
import { TFunction } from 'i18next';
import { useEffect, useRef, useState } from 'react';
import { PANEL_NAME_AUTO_SAVE_DELAY_MS } from '@/constants';

export type PanelNameProps = {
  t: TFunction<'SpacePanelEditor', undefined>;
  canEdit: boolean;
  name: string;
  setName: (name: string) => void;
};

export function PanelName({ t, canEdit, name, setName }: PanelNameProps) {
  const [editMode, setEditMode] = useState(false);
  const [internalName, setInternalName] = useState(name);
  const inputRef = useRef<HTMLInputElement>(null);
  const saveTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  useEffect(() => {
    if (editMode) inputRef.current?.focus();
  }, [editMode]);

  // Sync internal name with prop name when it changes externally
  useEffect(() => {
    setInternalName(name);
  }, [name]);

  // Cleanup timeout on unmount
  useEffect(() => {
    return () => {
      if (saveTimeoutRef.current) {
        clearTimeout(saveTimeoutRef.current);
      }
    };
  }, []);

  const saveName = () => {
    const trimmedName = internalName.trim();
    if (trimmedName !== name) {
      setName(trimmedName);
    }
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newValue = e.target.value;
    setInternalName(newValue);

    // Clear existing timeout
    if (saveTimeoutRef.current) {
      clearTimeout(saveTimeoutRef.current);
    }

    // Set new timeout for auto-save
    saveTimeoutRef.current = setTimeout(() => {
      const trimmedName = newValue.trim();
      if (trimmedName !== name) {
        setName(trimmedName);
      }
    }, PANEL_NAME_AUTO_SAVE_DELAY_MS);
  };

  const handleBlur = () => {
    // Clear any pending auto-save timeout
    if (saveTimeoutRef.current) {
      clearTimeout(saveTimeoutRef.current);
    }

    // Save immediately on blur
    saveName();
    setEditMode(false);
  };

  const onKeyDown = (e: React.KeyboardEvent) => {
    executeOnKeyStroke(
      e,
      () => {
        // Clear any pending auto-save timeout
        if (saveTimeoutRef.current) {
          clearTimeout(saveTimeoutRef.current);
        }
        saveName();
        setEditMode(false);
      },
      () => {
        // On Escape, revert to original name
        setInternalName(name);
        setEditMode(false);
      },
    );
  };

  return (
    <div>
      {editMode && canEdit ? (
        <Input
          ref={inputRef}
          className="border-b border-transparent !border-b-white focus:!border-transparent focus:rounded-md font-bold text-text-primary text-[14px]/[30px] placeholder:text-neutral-300 placeholder:font-medium rounded-none"
          value={internalName}
          onChange={handleChange}
          onKeyDown={onKeyDown}
          onBlur={handleBlur}
          placeholder={t('panel_name_hint')}
        />
      ) : (
        <div
          role="button"
          tabIndex={0}
          className="flex w-full items-center min-h-[30px] cursor-text"
          onClick={() => setEditMode(true)}
          onKeyDown={(e) => e.key === 'Enter' && setEditMode(true)}
        >
          {name ? (
            <div className="font-bold text-text-primary text-[14px]/[30px]">
              {name}
            </div>
          ) : (
            <div className="font-medium text-neutral-700 text-[14px]/[30px]">
              {canEdit ? t('panel_name_hint') : ''}
            </div>
          )}
        </div>
      )}
    </div>
  );
}
