export interface RemovePostRequest {
  delete: object;
}

export function removePostRequest(): RemovePostRequest {
  return {
    delete: {},
  };
}
