import { useMemo, useState } from 'react';
import { SpacePanelQuota } from '../types/space-panels-response';
import { PanelAttribute } from '../types/panel-attribute';
import { Input } from '@/components/ui/input';
import { Trash2 } from 'lucide-react';
import { TFunction } from 'i18next';

export type PanelTableProps = {
  t: TFunction<'SpacePanelEditor', undefined>;
  canEdit: boolean;
  panel_quotas: SpacePanelQuota[];
  onChangeQuota?: (row: number, next: number) => void;
  onDelete?: (row: number) => void;
};

const groupLabel = (
  a: PanelAttribute,
  t: TFunction<'SpacePanelEditor', undefined>,
) => {
  if (a.type === 'collective_attribute') {
    if (a.value === 'university') return t('university');
    return 'Group';
  }
  if (a.type === 'verifiable_attribute') {
    if (a.value === 'age') return t('age');
    if (a.value === 'gender') return t('gender');
    return 'Verifiable Attribute';
  }
  return 'None';
};

const extractValueLabel = (
  sk: string,
  t: TFunction<'SpacePanelEditor', undefined>,
): string => {
  const match = sk.match(/#gender:(\w+)$/);
  if (match) {
    const val = match[1];
    if (val === 'male') return t('male');
    if (val === 'female') return t('female');
    return val.charAt(0).toUpperCase() + val.slice(1);
  }
  const cityMatch = sk.match(/#collective_attribute:(\w+)#(\w+)/);
  if (cityMatch) return cityMatch[2];
  return '—';
};

export function PanelTable({
  t,
  canEdit,
  panel_quotas,
  onChangeQuota,
  onDelete,
}: PanelTableProps) {
  const [editing, setEditing] = useState<Record<number, string>>({});

  const total = useMemo(
    () => panel_quotas.reduce((sum, r) => sum + (r.quotas ?? 0), 0),
    [panel_quotas],
  );

  const pct = (n: number) =>
    total > 0 ? Math.round((n / total) * 1000) / 10 : 0;

  const hasDirty = (idx: number) =>
    Object.prototype.hasOwnProperty.call(editing, idx);

  const commit = (idx: number, fallback: number) => {
    if (!hasDirty(idx)) return;

    const raw = (editing[idx] ?? '').trim();
    const parsed = raw === '' ? fallback : Number(raw);
    const val = Number.isFinite(parsed) ? parsed : fallback;

    onChangeQuota?.(idx, val);

    setEditing((m) => {
      const { [idx]: _, ...rest } = m;
      return rest;
    });
  };

  return (
    <table className="w-full border border-input-box-border rounded-xl overflow-hidden text-sm">
      <thead className="bg-muted text-text-secondary">
        <tr>
          <th className="py-3 px-4 text-left">{t('attribute_groups')}</th>
          <th className="py-3 px-4 text-left">{t('attributes')}</th>
          <th className="py-3 px-4 text-right">{t('ratio')}</th>
          <th className="py-3 px-4 text-center">{t('total_quotas')}</th>
          <th className="py-3 px-4 text-right">{t('delete')}</th>
        </tr>
      </thead>

      <tbody>
        {panel_quotas.map((row, idx) => {
          const q = row.quotas ?? 0;
          const show = editing[idx] ?? String(q);

          return (
            <tr key={idx} className="border-t border-input-box-border">
              <td className="px-4 py-3">
                {groupLabel(row.attributes as PanelAttribute, t)}
              </td>

              <td className="px-4 py-3">
                <span className="inline-flex items-center rounded-md bg-neutral-700 light:bg-neutral-500 text-white px-2 py-1 text-xs dark:bg-neutral-200 dark:text-neutral-900">
                  {extractValueLabel(row.sk, t)}
                </span>
              </td>

              <td className="px-4 py-3 text-right">{pct(q).toFixed(1)}</td>

              <td className="px-4 py-3 text-right ">
                <Input
                  type="text"
                  inputMode="numeric"
                  pattern="[0-9]*"
                  value={show}
                  disabled={!canEdit}
                  onChange={(e) =>
                    setEditing((m) => ({
                      ...m,
                      [idx]: e.target.value.replace(/\D+/g, ''),
                    }))
                  }
                  onKeyDown={(e) => {
                    if (e.key === 'Enter') commit(idx, q);
                    if (e.key === 'Escape')
                      setEditing((m) => ({ ...m, [idx]: String(q) }));
                  }}
                  onBlur={() => commit(idx, q)}
                  className="h-8 text-left"
                />
              </td>

              <td className="px-4 py-3 text-right">
                <button
                  type="button"
                  disabled={!canEdit}
                  onClick={() => onDelete?.(idx)}
                >
                  <Trash2 className="[&>path]:stroke-neutral-500 [&>line]:stroke-neutral-500" />
                </button>
              </td>
            </tr>
          );
        })}
      </tbody>
    </table>
  );
}
