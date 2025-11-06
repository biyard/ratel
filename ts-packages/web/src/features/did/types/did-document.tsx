/**
 * DID Document structure based on W3C DID Core specification
 */

export type ServiceEndpoint = string | string[] | Record<string, unknown>;

export type ServiceType = string | string[];

export interface Service {
  id: string;
  type: ServiceType;
  serviceEndpoint: ServiceEndpoint;
}

export type DidContext = string | string[];

export type DidController = string | string[];

export interface VerificationMethod {
  id: string;
  type: string;
  controller: string;
  publicKeyJwk?: Record<string, unknown>;
  publicKeyMultibase?: string;
}

export type VerificationRelationship = string | VerificationMethod;

export interface DidDocument {
  '@context': DidContext;
  id: string;
  alsoKnownAs?: string[];
  controller?: DidController;
  verificationMethod?: VerificationMethod[];
  authentication?: VerificationRelationship[];
  assertionMethod?: VerificationRelationship[];
  keyAgreement?: VerificationRelationship[];
  capabilityInvocation?: VerificationRelationship[];
  capabilityDelegation?: VerificationRelationship[];
  service?: Service[];
  created?: string;
  updated?: string;
}
