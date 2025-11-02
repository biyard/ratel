import { Input } from '@/components/ui/input';
import { executeOnKeyStroke } from '@/utils/key-event-handle';
import { TFunction } from 'i18next';
import { useEffect, useRef, useState } from 'react';

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

  useEffect(() => {
    if (editMode) inputRef.current?.focus();
  }, [editMode]);

  // Sync internalName with name prop when it changes
  useEffect(() => {
    setInternalName(name);
  }, [name]);

  const onKeyDown = (e: React.KeyboardEvent) => {
    executeOnKeyStroke(
      e,
      () => {
        setName(internalName.trim());
        setEditMode(false);
      },
      () => setEditMode(false),
    );
  };

  const handleBlur = () => {
    // Save the internalName before exiting edit mode
    if (internalName.trim() !== name) {
      setName(internalName.trim());
    }
    setEditMode(false);
  };

  return (
    <div>
      {editMode && canEdit ? (
        <Input
          ref={inputRef}
          className="border-b border-transparent !border-b-white focus:!border-transparent focus:rounded-md font-bold text-text-primary text-[14px]/[30px] placeholder:text-neutral-300 placeholder:font-medium rounded-none"
          value={internalName}
          onChange={(e) => setInternalName(e.target.value)}
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
