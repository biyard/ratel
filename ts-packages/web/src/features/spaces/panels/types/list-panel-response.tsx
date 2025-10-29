import { SpacePanelResponse } from './space-panel-response';

export class ListPanelResponse {
  panels: SpacePanelResponse[];
  bookmark: string | null | undefined;

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    const rows = Array.isArray(json.panels) ? json.panels : [];
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    this.panels = rows.map((d: any) => new SpacePanelResponse(d));
    this.bookmark = json.bookmark;
  }
}
