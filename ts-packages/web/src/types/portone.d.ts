// PortOne (Iamport) Type Definitions
declare global {
  interface Window {
    IMP?: IMP;
  }
}

export interface IMP {
  init: (userCode: string) => void;
  certification: (
    params: CertificationParams,
    callback?: (response: CertificationResponse) => void
  ) => void;
}

export interface CertificationParams {
  merchant_uid: string; // 주문번호 (고유값)
  company?: string; // 회사명 또는 URL
  carrier?: string; // 통신사 (SKT, KT, LGU)
  name?: string; // 이름
  phone?: string; // 전화번호
  min_age?: number; // 최소 연령
  popup?: boolean; // 팝업 여부
  pg?: string; // PG사 (inicis, danal 등)
}

export interface CertificationResponse {
  success: boolean;
  imp_uid: string | null; // 포트원 고유번호
  merchant_uid: string; // 주문번호
  pg_provider?: string; // PG사
  pg_type?: string; // 인증 방식
  error_code?: string; // 에러 코드
  error_msg?: string; // 에러 메시지
}

export {};
