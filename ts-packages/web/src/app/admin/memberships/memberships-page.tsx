import { useMembershipsPageController } from './use-memberships-page-controller';
import { useMembershipsI18n } from '../../../features/membership/i18n';
import { MembershipTable } from '../../../features/membership/components/membership-table';
import { MembershipForm } from '../../../features/membership/components/membership-form';
import { DeleteMembershipDialog } from '../../../features/membership/components/delete-membership-dialog';

export function MembershipsPage() {
  const ctrl = useMembershipsPageController();
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
    <div className="p-6 mx-auto w-full max-w-desktop">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-3xl font-bold">{i18n.title}</h1>
        <button
          onClick={ctrl.openCreateForm}
          className="py-2 px-4 text-white bg-blue-500 rounded transition-colors hover:bg-blue-600"
        >
          {i18n.createNew}
        </button>
      </div>

      <div className="bg-white rounded-lg shadow dark:bg-gray-800">
        <MembershipTable
          memberships={ctrl.memberships}
          onEdit={ctrl.openEditForm}
          onDelete={ctrl.openDeleteConfirm}
        />
      </div>

      {ctrl.isFormOpen && (
        <MembershipForm
          membership={ctrl.editingMembership}
          onSubmit={
            ctrl.editingMembership
              ? ctrl.handleUpdateMembership
              : ctrl.handleCreateMembership
          }
          onCancel={ctrl.closeForm}
          isSubmitting={ctrl.isSubmitting}
        />
      )}

      {ctrl.deleteConfirmMembership && (
        <DeleteMembershipDialog
          membership={ctrl.deleteConfirmMembership}
          onConfirm={ctrl.handleDeleteMembership}
          onCancel={ctrl.closeDeleteConfirm}
          isDeleting={ctrl.isSubmitting}
        />
      )}
    </div>
  );
}
