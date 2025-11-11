export class SpaceRequirement {
  public pk: string;
  public sk: string;
  public order: number;
  public related_pk: string;
  public related_sk: string;
  public typ: SpaceRequirementType;

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    this.pk = json.pk;
    this.sk = json.sk;
    this.order = json.order;
    this.related_pk = json.related_pk;
    this.related_sk = json.related_sk;
    this.typ = json.typ;
  }
}

export enum SpaceRequirementType {
  PrePoll = 'PRE_POLL',
}
