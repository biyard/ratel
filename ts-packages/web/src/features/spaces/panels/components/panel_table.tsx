import { useMemo, useState } from 'react';
import { PanelAttributeType } from '../types/panel-attribute';
import { TFunction } from 'i18next';
import { EditDeletePanel } from '@/components/icons';
import { SpacePanel } from '../types/space-panel';

export type PanelTableProps = {
  t: TFunction<'SpacePanelEditor', undefined>;
  canEdit: boolean;
  panels: SpacePanel[];
  onChangeQuota?: (sk: string, quota: number) => void;
  onDelete?: (pk: string, sk: string) => void;
};

export function PanelTable({
  t,
  canEdit,
  panels,
  onChangeQuota,
  onDelete,
}: PanelTableProps) {
  const [editing, setEditing] = useState<Record<string, string>>({});

  // Filter to show only VerifiableAttribute entries
  const filteredQuotas = useMemo(
    () =>
      panels.filter(
        (q) => q.attributes.type === PanelAttributeType.VerifiableAttribute,
      ),
    [panels],
  );

  const total = useMemo(
    () => filteredQuotas.reduce((sum, r) => sum + (r.quotas ?? 0), 0),
    [filteredQuotas],
  );

  const pct = (n: number) =>
    total > 0 ? Math.round((n / total) * 1000) / 10 : 0;

  const commit = (sk: string, fallback: number) => {
    const raw = (editing[sk] ?? '').trim();
    const parsed = raw === '' ? fallback : Number(raw);
    const val = Number.isFinite(parsed) ? parsed : fallback;

    onChangeQuota?.(sk, val);

    setEditing((m) => {
      const { [sk]: _, ...rest } = m;
      return rest;
    });
  };

  return (
    <table className="overflow-hidden w-full text-sm rounded-xl border border-input-box-border">
      <thead className="bg-muted text-[var(--color-panel-table-header)]">
        <tr>
          <th className="py-3 px-4 text-left">{t('attribute_groups')}</th>
          <th className="py-3 px-4 text-left">{t('attributes')}</th>
          <th className="py-3 px-4 text-right">{t('ratio')}</th>
          <th className="py-3 px-4 text-center">{t('total_quotas')}</th>
          <th className="py-3 px-4 text-right"></th>
        </tr>
      </thead>

      <tbody>
        {filteredQuotas.map((quota) => {
          const attributeGroup = quota.toPanelOption().toString().toLowerCase();
          const attributeValue = quota.toPanelValue().toLowerCase();

          return (
            <tr
              key={`${quota.pk}-${quota.sk}`}
              className="border-t border-input-box-border hover:bg-muted/50"
            >
              <td className="py-3 px-4 font-medium text-left">
                {t(attributeGroup)}
              </td>
              <td className="py-3 px-4 text-left">{t(attributeValue)}</td>
              <td className="py-3 px-4 text-right text-text-secondary">
                {pct(quota.quotas ?? 0)}%
              </td>
              <td className="py-3 px-4 text-center">
                {canEdit ? (
                  <input
                    type="text"
                    className="py-1 px-2 w-20 text-center rounded border border-input-box-border"
                    value={
                      editing[quota.sk] !== undefined
                        ? editing[quota.sk]
                        : String(quota.quotas ?? 0)
                    }
                    onChange={(e) =>
                      setEditing((m) => ({ ...m, [quota.sk]: e.target.value }))
                    }
                    onBlur={() => commit(quota.sk, quota.quotas ?? 0)}
                    onKeyDown={(e) => {
                      if (e.key === 'Enter') {
                        commit(quota.sk, quota.quotas ?? 0);
                      } else if (e.key === 'Escape') {
                        setEditing((m) => {
                          const { [quota.sk]: _, ...rest } = m;
                          return rest;
                        });
                      }
                    }}
                  />
                ) : (
                  <span>{quota.quotas ?? 0}</span>
                )}
              </td>
              <td className="py-3 px-4 text-right">
                {canEdit && (
                  <button
                    type="button"
                    onClick={() => onDelete?.(quota.pk, quota.sk)}
                    className="p-1 rounded transition-colors hover:bg-muted panel-delete-btn"
                    aria-label="Delete attribute"
                  >
                    <EditDeletePanel className="w-6 h-6" />
                  </button>
                )}
              </td>
            </tr>
          );
        })}
        {filteredQuotas.length === 0 && (
          <tr>
            <td colSpan={5} className="py-8 text-center text-text-secondary">
              {t('no_attributes')}
            </td>
          </tr>
        )}
      </tbody>
    </table>
  );
}
