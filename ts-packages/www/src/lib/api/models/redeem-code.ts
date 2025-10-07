export interface RedeemCode {
  id: number;
  created_at: number;
  updated_at: number;
  meta_id: number;
  user_id: number;
  codes: string[];
  used: number[];
}

export interface RedeemCodeRequest {
  use_code: {
    code: string;
  };
}
export function redeemCodeRequest(code: string): RedeemCodeRequest {
  return {
    use_code: {
      code,
    },
  };
}
