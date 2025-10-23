import { useAdminPageController } from './use-admin-page-controller';
import { useAdminI18n } from './admin-page-i18n';
import { NavLink } from 'react-router';
import { route } from '@/route';

export function AdminPage() {
  const ctrl = useAdminPageController();
  const i18n = useAdminI18n();

  return (
    <div className="w-full max-w-desktop mx-auto p-6">
      <h1 className="text-3xl font-bold mb-6">{i18n.title}</h1>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        <NavLink
          to={route.adminMemberships()}
          className="block p-6 bg-white dark:bg-gray-800 rounded-lg shadow hover:shadow-lg transition-shadow border border-gray-200 dark:border-gray-700"
        >
          <h2 className="text-xl font-semibold mb-2">{i18n.memberships}</h2>
          <p className="text-gray-600 dark:text-gray-400">
            {i18n.membershipsDesc}
          </p>
        </NavLink>

        {/* Add more admin sections here as needed */}
      </div>
    </div>
  );
}
