export enum Gender {
  Male = 'male',
  Female = 'female',
}

export enum AgeType {
  Specific = 'specific',
  Range = 'range',
}

export type SpecificAge = number;

export type AgeRange = {
  inclusive_min: number;
  inclusive_max: number;
};

export type AgeAttribute = {
  type: AgeType;
  value: SpecificAge | AgeRange;
};

export enum VerifiableAttributeType {
  Gender = 'gender',
}

export type VerifiableAttribute = {
  type: VerifiableAttributeType;
  value: Gender | AgeAttribute;
};

export type VerifiableAttributeWithQuota = VerifiableAttribute & {
  quota: number;
};

export enum PanelAttributeType {
  CollectiveAttribute = 'collective_attribute',
  VerifiableAttribute = 'verifiable_attribute',
}

export enum CollectiveAttribute {
  University = 'university',
  Age = 'age',
  Gender = 'gender',
}

export type PanelAttribute = {
  type: PanelAttributeType;
  value: VerifiableAttribute | CollectiveAttribute;
};

export type PanelAttributeWithQuota = {
  type: PanelAttributeType;
  value: VerifiableAttributeWithQuota | CollectiveAttribute;
};
