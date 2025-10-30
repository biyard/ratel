import { parseAttribute, Attribute } from './answer-type';

export class SpacePanelResponse {
  pk: string;

  name: string;
  quotas: number;
  participants: number;
  attributes: Attribute[];

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    this.pk = json.pk;
    this.name = json.name;
    this.quotas = json.quotas;
    this.participants = json.participants;
    const raws = Array.isArray(json.attributes) ? json.attributes : [];
    this.attributes = raws
      .map(parseAttribute)
      .filter((a): a is Attribute => a !== null);
  }
}
