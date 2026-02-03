export class NetworkEdgeRow {
  public source: string;
  public target: string;
  public weight: number;

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    this.source = String(json?.source ?? '');
    this.target = String(json?.target ?? '');
    this.weight = Number(json?.weight ?? 0);
  }
}
