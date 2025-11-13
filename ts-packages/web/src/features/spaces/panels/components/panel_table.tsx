import { useMemo, useState } from 'react';
import { PanelAttributeType } from '../types/panel-attribute';
import { TFunction } from 'i18next';
import { Trash2 } from 'lucide-react';
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
  const [editing, setEditing] = useState<Record<number, string>>({});

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

  return (
    <table className="overflow-hidden w-full text-sm rounded-xl border border-input-box-border">
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
        {filteredQuotas.map((quota, idx) => {
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
                {quota.quotas}%
              </td>
              <td className="py-3 px-4 text-center">
                {canEdit ? (
                  <input
                    type="text"
                    className="py-1 px-2 w-20 text-center rounded border border-input-box-border"
                    value={quota.quotas}
                    onChange={(e) =>
                      onChangeQuota?.(quota.sk, Number(e.target.value))
                    }
                    onKeyDown={(e) => {
                      if (e.key === 'Enter' || e.key === 'Tab') {
                        onChangeQuota?.(quota.sk, Number(e.target.value));
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
                    className="p-1 text-red-600 rounded transition-colors hover:text-red-700 hover:bg-red-50"
                    aria-label="Delete attribute"
                  >
                    <Trash2 className="w-4 h-4" />
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
