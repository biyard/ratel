import { useEffect, useState } from 'react';
import type { CertificationResponse } from '@/types/portone';

interface UseIdentityVerificationOptions {
  userCode: string; // 포트원 가맹점 식별코드
  onSuccess?: (response: CertificationResponse) => void;
  onError?: (response: CertificationResponse) => void;
}

interface UseIdentityVerificationResult {
  isReady: boolean;
  verify: (options?: {
    name?: string;
    phone?: string;
    minAge?: number;
  }) => Promise<void>;
  isVerifying: boolean;
}

export function useIdentityVerification({
  userCode,
  onSuccess,
  onError,
}: UseIdentityVerificationOptions): UseIdentityVerificationResult {
  const [isReady, setIsReady] = useState(false);
  const [isVerifying, setIsVerifying] = useState(false);

  useEffect(() => {
    // PortOne SDK 로드 확인
    const checkIMP = setInterval(() => {
      if (window.IMP) {
        window.IMP.init(userCode);
        setIsReady(true);
        clearInterval(checkIMP);
      }
    }, 100);

    return () => clearInterval(checkIMP);
  }, [userCode]);

  const verify = async (options?: {
    name?: string;
    phone?: string;
    minAge?: number;
  }) => {
    if (!window.IMP || !isReady) {
      console.error('PortOne SDK is not ready');
      return;
    }

    setIsVerifying(true);

    const merchantUid = `cert_${new Date().getTime()}`;

    window.IMP.certification(
      {
        merchant_uid: merchantUid,
        pg: 'inicis_unified',
        name: options?.name,
        phone: options?.phone,
        min_age: options?.minAge,
        popup: false,
      },
      (response: CertificationResponse) => {
        setIsVerifying(false);

        if (response.success) {
          console.log('본인인증 성공:', response);
          onSuccess?.(response);
        } else {
          console.error('본인인증 실패:', response);
          onError?.(response);
        }
      },
    );
  };

  return {
    isReady,
    verify,
    isVerifying,
  };
}
