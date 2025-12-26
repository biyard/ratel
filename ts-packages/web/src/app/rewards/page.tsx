import { Suspense } from 'react';
import { ErrorBoundary } from '@/components/error-boundary';
import RewardsPage from './rewards-page';
import { useMyRewardsI18n } from './rewards-page-i18n';

function RewardsLoadingFallback() {
  return (
    <div className="w-full max-w-desktop mx-auto px-4 py-8">
      <div className="text-center text-foreground">Loading...</div>
    </div>
  );
}

function RewardsErrorFallback({ msg }: { msg: string }) {
  return (
    <div className="w-full max-w-desktop mx-auto px-4 py-8">
      <div className="bg-card-bg border border-card-border rounded-lg p-8">
        <div className="text-center text-destructive">{msg}</div>
      </div>
    </div>
  );
}

export default function RewardsPageWrapper() {
  const i18n = useMyRewardsI18n();
  return (
    <ErrorBoundary fallback={<RewardsErrorFallback msg={i18n.error} />}>
      <Suspense fallback={<RewardsLoadingFallback />}>
        <RewardsPage />
      </Suspense>
    </ErrorBoundary>
  );
}
