import { PanelAttributeOptions } from '../pages/creator/panel-attribute-options';
import {
  PanelAttribute,
  PanelAttributeType,
  VerifiableAttribute,
} from './panel-attribute';

export class SpacePanel {
  public pk: string;
  public sk: string;
  public quotas: number;
  public remains: number;
  public attributes: PanelAttribute;

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  constructor(json: any) {
    this.pk = json.pk;
    this.sk = json.sk;
    this.quotas = json.quotas;
    this.remains = json.remains;
    this.attributes = json.attributes;
  }

  toPanelOption(): PanelAttributeOptions {
    switch (this.attributes.type) {
      case PanelAttributeType.CollectiveAttribute:
        return `${this.attributes.value}` as PanelAttributeOptions;
      case PanelAttributeType.VerifiableAttribute:
        return `${(this.attributes.value as VerifiableAttribute).type}` as PanelAttributeOptions;
    }
  }

  isOption(option: PanelAttributeOptions): boolean {
    return option === this.toPanelOption();
  }

  toPanelValue(): string {
    switch (this.attributes.type) {
      case PanelAttributeType.CollectiveAttribute:
        return `${this.attributes.value}`;
      case PanelAttributeType.VerifiableAttribute:
        return `${(this.attributes.value as VerifiableAttribute).value}`;
    }
  }
}
