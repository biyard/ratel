export const NoEd25519KeyPair = new Error('no Ed25519KeyPair has been set');

// Validation
export const InvalidTooShort = new Error('too short');
export const InvalidLowerAlphaNumeric = new Error(
  'must be combination of lower alphanumeric characters, dashes, and underscores',
);
export const InvalidDuplicatedUsername = new Error('username already exists');

export const ApiCallFailure = (msg: string) =>
  new Error(`API call failed: ${msg}`);
