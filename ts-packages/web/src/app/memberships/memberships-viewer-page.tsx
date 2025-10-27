import { useMembershipsI18n } from '@/features/membership/i18n';
import { useMembershipsViewerPageController } from './use-memberships-viewer-page-controller';

export function MembershipsViewerPage() {
  const ctrl = useMembershipsViewerPageController();
  const i18n = useMembershipsI18n();

  if (ctrl.isLoading) {
    return (
      <div className="p-6 mx-auto w-full max-w-desktop">
        <div className="py-8 text-center">{i18n.loading}</div>
      </div>
    );
  }

  if (ctrl.error) {
    return (
      <div className="p-6 mx-auto w-full max-w-desktop">
        <div className="py-8 text-center text-red-500">
          {i18n.loadError}: {ctrl.error.message}
        </div>
      </div>
    );
  }

  return (
    <div className="w-full max-w-desktop mx-auto p-6">
      <h1 className="text-3xl font-bold mb-6">Membership</h1>
    </div>
  );
}
