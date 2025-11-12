import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import usePanelSpace from '../../hooks/use-panel-space';
import { usePopup } from '@/lib/contexts/popup-service';
import { TFunction } from 'i18next';
import { useTranslation } from 'react-i18next';
import { useUpdatePanelMutation } from '../../hooks/use-update-panel-mutation';
import { Attribute, PanelAttribute } from '../../types/panel-attribute';
import { useCreatePanelQuotaMutation } from '../../hooks/use-create-panel-quota-mutation';
import { useDeletePanelQuotaMutation } from '../../hooks/use-delete-panel-quota-mutation';
import { useUpdatePanelQuotaMutation } from '../../hooks/use-update-panel-quota-mutation copy';
import { SpacePanelResponse } from '../../types/space-panels-response';
import { showErrorToast } from '@/lib/toast';

export class SpacePanelEditorController {
  constructor(
    public spacePk: string,
    public space: Space,
    public panel: SpacePanelResponse,
    public popup: ReturnType<typeof usePopup>,
    public t: TFunction<'SpacePanelEditor', undefined>,

    public updatePanel: ReturnType<typeof useUpdatePanelMutation>,
    public createPanelQuota: ReturnType<typeof useCreatePanelQuotaMutation>,
    public deletePanelQuota: ReturnType<typeof useDeletePanelQuotaMutation>,
    public updatePanelQuota: ReturnType<typeof useUpdatePanelQuotaMutation>,
  ) {}

  handleUpdateAttributeQuota = async (row: number, next: number) => {
    const q = this.panel.panel_quotas?.[row];
    if (!q) return;

    const total_quotas = Number(this.panel.quotas ?? 0);
    const panel_quotas = this.panel.panel_quotas ?? [];

    const sumExceptRow = panel_quotas.reduce(
      (acc, r, i) => acc + (i === row ? 0 : Number(r.quotas ?? 0)),
      0,
    );
    const newSum = sumExceptRow + Number(next ?? 0);

    if (newSum < total_quotas) {
      const msg = `The total of all quotas cannot exceed the overall quota.`;
      showErrorToast(msg);
      return;
    }

    const attribute = q.attributes as PanelAttribute;
    const sk = (q as unknown as { sk?: string }).sk ?? '';

    let value: Attribute | null = null;

    if (
      attribute.type === 'verifiable_attribute' &&
      attribute.value === 'gender'
    ) {
      const m = sk.match(/#gender:(male|female)$/);
      if (m?.[1] === 'male') value = { answer_type: 'gender', male: {} };
      if (m?.[1] === 'female') value = { answer_type: 'gender', female: {} };
    }

    if (!value) return;

    await this.updatePanelQuota.mutateAsync({
      spacePk: this.spacePk,
      quotas: next,
      attribute,
      value,
    });
  };

  handleDeleteAttributeQuota = async (row: number) => {
    const q = this.panel.panel_quotas?.[row];
    if (!q) return;

    const attribute = q.attributes as PanelAttribute;
    const sk = (q as unknown as { sk?: string }).sk ?? '';

    let value: Attribute | null = null;

    if (
      attribute.type === 'verifiable_attribute' &&
      attribute.value === 'gender'
    ) {
      const m = sk.match(/#gender:(male|female)$/);
      if (m?.[1] === 'male') value = { answer_type: 'gender', male: {} };
      if (m?.[1] === 'female') value = { answer_type: 'gender', female: {} };
    } else if (
      attribute.type === 'verifiable_attribute' &&
      attribute.value === 'age'
    ) {
      const m = sk.match(/#age:(\d+)(?:-(\d+))?$/);
      if (m) {
        value = m[2]
          ? {
              answer_type: 'age',
              range: {
                inclusive_min: Number(m[1]),
                inclusive_max: Number(m[2]),
              },
            }
          : { answer_type: 'age', specific: Number(m[1]) };
      }
    } else {
      return;
    }

    if (!value) return;

    await this.deletePanelQuota.mutateAsync({
      spacePk: this.spacePk,
      attribute,
      value,
    });
  };

  handleUpdateValues = async (v: PanelAttribute[]) => {
    const prevHasGender = this.panel.attributes.some(
      (a) => a.type === 'verifiable_attribute' && a.value === 'gender',
    );
    const newHasGender = v.some(
      (a) => a.type === 'verifiable_attribute' && a.value === 'gender',
    );

    await this.updatePanel.mutateAsync({
      spacePk: this.spacePk,
      quotas: this.panel.quotas,
      attributes: v,
    });

    if (!prevHasGender && newHasGender) {
      const total =
        Array.isArray(this.panel.quotas) && this.panel.quotas.length > 0
          ? this.panel.quotas[0]
          : this.panel.quotas;

      const male = Math.floor(total / 2);
      const female = total - male;

      await this.createPanelQuota.mutateAsync({
        spacePk: this.spacePk,
        quotas: [male, female],
        attributes: [
          { type: 'verifiable_attribute', value: 'gender' },
          { type: 'verifiable_attribute', value: 'gender' },
        ],
        values: [
          { answer_type: 'gender', male: {} },
          { answer_type: 'gender', female: {} },
        ],
      });
    }

    if (prevHasGender && !newHasGender) {
      await Promise.all([
        this.deletePanelQuota.mutateAsync({
          spacePk: this.spacePk,
          attribute: { type: 'verifiable_attribute', value: 'gender' },
          value: { answer_type: 'gender', male: {} },
        }),
        this.deletePanelQuota.mutateAsync({
          spacePk: this.spacePk,
          attribute: { type: 'verifiable_attribute', value: 'gender' },
          value: { answer_type: 'gender', female: {} },
        }),
      ]);
    }
  };

  handleUpdateQuota = async (quotas: number) => {
    await this.updatePanel.mutateAsync({
      spacePk: this.spacePk,
      quotas,
      attributes: this.panel.attributes,
    });
  };
}

export function useSpacePanelEditorController(spacePk: string) {
  const popup = usePopup();
  const { t } = useTranslation('SpacePanelEditor');
  const { data: space } = useSpaceById(spacePk);
  const { data: panel } = usePanelSpace(spacePk);

  const updatePanel = useUpdatePanelMutation();
  const createPanelQuota = useCreatePanelQuotaMutation();
  const deletePanelQuota = useDeletePanelQuotaMutation();
  const updatePanelQuota = useUpdatePanelQuotaMutation();

  return new SpacePanelEditorController(
    spacePk,
    space,
    panel,
    popup,
    t,
    updatePanel,
    createPanelQuota,
    deletePanelQuota,
    updatePanelQuota,
  );
}
