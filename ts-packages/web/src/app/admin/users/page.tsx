import { useAdminsPageController } from './use-admins-page-controller';
import { useAdminsI18n } from './admins-page-i18n';
import { AdminTable } from '@/features/admin/components/admin-table';
import { PromoteAdminDialog } from '@/features/admin/components/promote-admin-dialog';
import { DemoteAdminDialog } from '@/features/admin/components/demote-admin-dialog';

export default function AdminUsersPage() {
  const ctrl = useAdminsPageController();
  const i18n = useAdminsI18n();

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
    <div className="p-6 mx-auto w-full max-w-desktop">
      <div className="flex justify-between items-center mb-6">
        <div>
          <h1 className="text-3xl font-bold mb-2">{i18n.title}</h1>
          <div className="h-1 w-20 bg-gradient-to-r from-blue-500 to-purple-500 rounded-full"></div>
        </div>
        <button
          onClick={ctrl.openPromoteDialog}
          className="py-2 px-4 text-white bg-blue-500 rounded transition-colors hover:bg-blue-600"
        >
          {i18n.promoteAdmin}
        </button>
      </div>

      <div className="bg-white rounded-lg shadow dark:bg-gray-800">
        <AdminTable admins={ctrl.admins} onDemote={ctrl.openDemoteDialog} />
      </div>

      <PromoteAdminDialog
        isOpen={ctrl.isPromoteDialogOpen}
        onClose={ctrl.closePromoteDialog}
        onSubmit={ctrl.handlePromoteToAdmin}
        isSubmitting={ctrl.isSubmitting}
        error={ctrl.actionError}
      />

      <DemoteAdminDialog
        isOpen={ctrl.isDemoteDialogOpen}
        admin={ctrl.selectedAdmin}
        onClose={ctrl.closeDemoteDialog}
        onConfirm={ctrl.handleDemoteAdmin}
        isSubmitting={ctrl.isSubmitting}
        error={ctrl.actionError}
      />
    </div>
  );
}
