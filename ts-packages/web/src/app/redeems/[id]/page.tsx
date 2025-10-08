'use client';

import Loading from '@/app/loading';
import { redeemCodeRequest } from '@/lib/api/models/redeem-code';
import { ratelApi } from '@/lib/api/ratel_api';
import { useApiCall } from '@/lib/api/use-send';
import { useEffect } from 'react';
import { useParams, useNavigate, useSearchParams } from 'react-router';

export default function RedeemPage() {
  const navigate = useNavigate();

  const params = useParams();
  const id = Number(params.id);

  const [searchParams] = useSearchParams();
  const code = searchParams.get('code') ?? undefined;

  const idStr = Array.isArray(id) ? id[0] : id;
  const metaId = parseInt(idStr, 10);

  const { post } = useApiCall();

  useEffect(() => {
    const fetchRedeemDetails = async () => {
      try {
        if (code === undefined || isNaN(metaId)) {
          navigate('/');
        }
        const response: { meta_id: number } = await post(
          ratelApi.redeems.useRedeemCode(metaId),
          redeemCodeRequest(code || ''),
        );
        if (!!response) {
          navigate(`/spaces/${response.meta_id}`);
        }
      } catch (error) {
        console.error('Failed to fetch redeem details:', error);
      }
    };

    fetchRedeemDetails();
  }, [metaId, code, post, navigate]);

  if (!id || !code || isNaN(id)) {
    return <p>Invalid request. Redirecting...</p>;
  }
  return (
    <div className="flex flex-col w-full h-screen items-center justify-center">
      <Loading />
    </div>
  );
}
