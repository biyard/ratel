import { PanelAttribute, parsePanelAttribute } from './panel-attribute';

export type SpacePanelQuota = {
  pk: string;
  sk: string;
  quotas: number;
  remains: number;
  attributes: PanelAttribute;
};

export class SpacePanelResponse {
  pk: string;

  quotas: number;
  attributes: PanelAttribute[];
  panel_quotas: SpacePanelQuota[];

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    this.pk = json.pk;
    this.quotas = json.quotas;

    // attributes: PanelAttribute[]
    const rawAttrs = Array.isArray(json.attributes) ? json.attributes : [];
    this.attributes = rawAttrs
      .map(parsePanelAttribute)
      .filter((a): a is PanelAttribute => a !== null);

    // panel_quotas: SpacePanelQuota[]
    const rawQuotas = Array.isArray(json.panel_quotas) ? json.panel_quotas : [];
    this.panel_quotas = rawQuotas
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      .map((q: any) => {
        const parsedAttr = parsePanelAttribute(q?.attributes);
        if (!parsedAttr) return null;
        return {
          pk: q.pk,
          sk: q.sk,
          quotas: q.quotas ?? 0,
          remains: q.remains ?? 0,
          attributes: parsedAttr,
        } as SpacePanelQuota;
      })
      .filter((q): q is SpacePanelQuota => q !== null);
  }
}
