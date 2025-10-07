export interface SubscribeRequest {
  chat_id: number;
  lang?: string;
}

export function subscribeRequest(
  chat_id: number,
  lang?: string,
): SubscribeRequest {
  return {
    chat_id,
    lang,
  };
}
