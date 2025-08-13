'use client';

import { useEffect, useState } from 'react';
import { useSearchParams } from 'next/navigation';

const KEY = 'referral_code';

function Handler() {
  const searchParams = useSearchParams();

  useEffect(() => {
    const referralCode = searchParams.get('referral');
    console.log('Referral code from search params:', referralCode);
    if (referralCode) {
      localStorage.setItem(
        KEY,
        JSON.stringify({
          code: referralCode,
          timestamp: Date.now(),
        }),
      );

      console.log('Referral code detected:', referralCode);
    }
  }, [searchParams]);

  return null;
}

export default function ReferralHandler() {
  return <Handler />;
}

export function useReferralInfo() {
  const [referralInfo, setReferralInfo] = useState<{
    code?: string;
    timestamp?: number;
  }>({});

  useEffect(() => {
    const referralInfoString = localStorage.getItem(KEY);
    if (referralInfoString) {
      try {
        const parsedInfo = JSON.parse(referralInfoString);
        setReferralInfo({
          code: parsedInfo.code,
          timestamp: parsedInfo.timestamp,
        });
      } catch (e) {
        console.error('Failed to parse referral info:', e);
        setReferralInfo({});
      }
    }
  }, []);

  return referralInfo;
}
