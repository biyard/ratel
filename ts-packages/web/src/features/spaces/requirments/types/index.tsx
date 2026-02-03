export class SpaceRequirement {
  public order: number;
  public related_pk: string;
  public related_sk: string;
  public typ: SpaceRequirementType;
  public responded: boolean;

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    this.order = json.order;
    this.related_pk = json.related_pk;
    this.related_sk = json.related_sk;
    this.typ = json.typ;
    this.responded = json.responded;
  }
}

export enum SpaceRequirementType {
  PrePoll = 'PRE_POLL',
}
