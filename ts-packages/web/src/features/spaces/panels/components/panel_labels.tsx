import { useEffect, useMemo, useRef, useState } from 'react';
import { PanelAttribute } from '../types/panel-attribute';
import { Clear } from '@/components/icons';
import { TFunction } from 'i18next';

const getValue = (p: PanelAttribute) => (p.type === 'none' ? 'none' : p.value);
const eq = (a: PanelAttribute, b: PanelAttribute) =>
  a.type === b.type && getValue(a) === getValue(b);

const UNIVERSITY: PanelAttribute = {
  type: 'collective_attribute',
  value: 'university',
};
const GENDER: PanelAttribute = {
  type: 'verifiable_attribute',
  value: 'gender',
};
const ALL_OPTIONS: PanelAttribute[] = [UNIVERSITY, GENDER];

const labelOf = (
  p: PanelAttribute,
  t: TFunction<'SpacePanelEditor', undefined>,
) => {
  if (p.type === 'collective_attribute') {
    if (p.value === 'university') return t('university');
    return t('attribute_groups');
  }
  if (p.type === 'verifiable_attribute') {
    if (p.value === 'age') return t('age');
    if (p.value === 'gender') return t('gender');
    return t('attribute_groups');
  }
  return '—';
};

type PanelLabelsProps = {
  canEdit: boolean;
  values: PanelAttribute[];
  setValues: (v: PanelAttribute[]) => void;
  placeholder?: string;
  t: TFunction<'SpacePanelEditor', undefined>;
};

export function PanelLabels({
  canEdit,
  values,
  setValues,
  placeholder = 'Attribute Groups',
  t,
}: PanelLabelsProps) {
  const [open, setOpen] = useState(false);
  const [anchorW, setAnchorW] = useState<number>();
  const anchorRef = useRef<HTMLDivElement>(null);
  const panelRef = useRef<HTMLDivElement>(null);

  const available = useMemo(
    () => ALL_OPTIONS.filter((opt) => !values.some((v) => eq(v, opt))),
    [values],
  );

  useEffect(() => {
    if (!open) return;
    const w = anchorRef.current?.getBoundingClientRect().width;
    if (w) setAnchorW(w);
  }, [open]);

  useEffect(() => {
    if (!open) return;
    const handler = (e: MouseEvent) => {
      if (
        panelRef.current &&
        !panelRef.current.contains(e.target as Node) &&
        anchorRef.current &&
        !anchorRef.current.contains(e.target as Node)
      ) {
        setOpen(false);
      }
    };
    document.addEventListener('mousedown', handler);
    return () => document.removeEventListener('mousedown', handler);
  }, [open]);

  useEffect(() => {
    if (available.length === 0 && open) setOpen(false);
  }, [available.length, open]);

  const toggle = (opt: PanelAttribute) => {
    const exists = values.some((v) => eq(v, opt));
    setValues(exists ? values.filter((v) => !eq(v, opt)) : [...values, opt]);
  };

  const clearAll = () => setValues([]);

  return (
    <div className="relative w-full">
      <div
        ref={anchorRef}
        role="button"
        tabIndex={0}
        onClick={() => canEdit && available.length > 0 && setOpen(true)}
        onKeyDown={(e) =>
          e.key === 'Enter' && canEdit && available.length > 0 && setOpen(true)
        }
        className="relative cursor-pointer flex w-full items-center justify-between min-h-[44px] rounded-lg px-4 py-2 bg-card-bg text-text-primary"
      >
        <div className="flex flex-wrap gap-2">
          {values.length === 0 ? (
            <span className="text-neutral-500">{placeholder}</span>
          ) : (
            values.map((v, i) => (
              <button
                key={i}
                type="button"
                className="inline-flex items-center gap-1 rounded-md bg-neutral-700 light:bg-neutral-500 text-white px-2 py-1 text-xs dark:bg-neutral-200 dark:text-neutral-900"
                onClick={(e) => {
                  e.stopPropagation();
                  toggle(v);
                  setOpen(false);
                }}
              >
                {labelOf(v, t)}
                <span className="opacity-70">
                  <Clear />
                </span>
              </button>
            ))
          )}
        </div>
        {values.length !== 0 && (
          <button
            type="button"
            className="ml-3 inline-flex h-6 w-6 items-center justify-center rounded-full light:bg-neutral-500 bg-neutral-700 text-white text-xs hover:bg-neutral-600 dark:bg-neutral-300 dark:text-neutral-900"
            onClick={(e) => {
              e.stopPropagation();
              clearAll();
            }}
            aria-label="Clear"
          >
            <Clear />
          </button>
        )}
      </div>

      {open && canEdit && available.length > 0 && (
        <div
          ref={panelRef}
          className="absolute z-50 mt-2 rounded-md bg-neutral-500 text-text-primary shadow-lg dark:border-neutral-700 dark:bg-neutral-900"
          style={{ width: anchorW ?? '100%' }}
        >
          <div className="max-h-72 overflow-auto rounded-b-md">
            <ul className="py-1">
              {available.map((opt, idx) => (
                <li key={idx}>
                  <button
                    type="button"
                    onClick={() => {
                      toggle(opt);
                      setOpen(false);
                    }}
                    className="flex w-full items-center justify-between px-4 py-2 text-left text-sm hover:bg-neutral-200/60 dark:hover:bg-neutral-800"
                  >
                    <span>{labelOf(opt, t)}</span>
                  </button>
                </li>
              ))}
            </ul>
          </div>
        </div>
      )}
    </div>
  );
}
