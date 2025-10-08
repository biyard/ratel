'use client';

import { useSearchParams } from 'react-router';
import { config } from '@/config';
import { apiFetch } from '@/lib/api/apiFetch';
import { Button } from '@/components/ui/button';

interface ApproveRequest {
  client_id: string;
  redirect_uri: string;
  scope: string;
  state: string;
}
interface ApproveResponse {
  redirect_url: string;
}
export default function OAuthLoginPage() {
  const [params] = useSearchParams();
  const approveRequest: ApproveRequest = {
    client_id: params.get('client_id') || '',
    redirect_uri: params.get('redirect_uri') || '',
    scope: params.get('scope') || '',
    state: params.get('state') || '',
  };
  const handle_approve = async () => {
    const res = await apiFetch<ApproveResponse>(
      `${config.api_url}/v2/oauth/approve`,
      {
        method: 'POST',
        credentials: 'include',
        body: JSON.stringify(approveRequest),
      },
    );

    if (res.data?.redirect_url) {
      window.location.href = res.data.redirect_url;
    }
  };
  return (
    <main className="absolute top-0 left-0 h-screen w-screen bg-neutral-80 flex flex-col items-center justify-center">
      <div className="flex flex-col size-100 p-8 rounded-lg bg-neutral-800 justify-center items-center gap-10">
        <Button onClick={() => handle_approve()}>Approve</Button>
      </div>
    </main>
  );
}
