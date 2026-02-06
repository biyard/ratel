import { useAdminMigrationsI18n } from './migrations-page-i18n';
import { useAdminMigrationsController } from './use-admin-migrations-controller';

export function MigrationsPage() {
  console.log('Hello');
  const i18n = useAdminMigrationsI18n();
  const ctrl = useAdminMigrationsController(i18n);

  if (ctrl.isLoading) {
    return (
      <div className="p-6 mx-auto w-full max-w-desktop">
        <div className="py-8 text-center">{i18n.running}</div>
      </div>
    );
  }

  return (
    <div className="p-6 mx-auto w-full max-w-desktop">
      <div className="mb-6">
        <h1 className="text-3xl font-bold">{i18n.title}</h1>
        <p className="mt-2 text-sm text-gray-500 dark:text-gray-400">
          {i18n.description}
        </p>
      </div>

      {!ctrl.isAdmin && (
        <div className="mb-6 rounded-md border border-red-200 bg-red-50 px-4 py-3 text-sm text-red-700 dark:border-red-900/50 dark:bg-red-950/40 dark:text-red-200">
          {i18n.failed}
        </div>
      )}

      <div className="bg-white dark:bg-gray-800 rounded-lg shadow border border-gray-200 dark:border-gray-700 p-6">
        <div className="flex items-center justify-between gap-4">
          <div>
            <div className="text-sm font-semibold text-gray-700 dark:text-gray-200">
              {i18n.runTeams}
            </div>
            {ctrl.lastStatus === 'success' && (
              <div className="mt-1 text-xs text-green-600 dark:text-green-400">
                {i18n.success}
              </div>
            )}
            {ctrl.lastStatus === 'failed' && (
              <div className="mt-1 text-xs text-red-600 dark:text-red-400">
                {i18n.failed}
              </div>
            )}
          </div>
          <button
            onClick={ctrl.runTeams}
            disabled={ctrl.isRunning}
            className="px-4 py-2 text-sm font-medium text-white bg-blue-500 rounded-md hover:bg-blue-600 disabled:opacity-60 disabled:cursor-not-allowed"
          >
            {ctrl.isRunning ? i18n.running : i18n.runTeams}
          </button>
        </div>
      </div>
    </div>
  );
}
