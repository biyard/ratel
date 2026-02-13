import { useAdminPageController } from './use-admin-page-controller';
import { useAdminI18n } from './admin-page-i18n';
import { NavLink } from 'react-router';
import { route } from '@/route';

export function AdminPage() {
  const _ctrl = useAdminPageController();
  const i18n = useAdminI18n();

  const adminCards = [
    {
      to: route.adminUsers(),
      title: i18n.users,
      description: i18n.usersDesc,
      icon: '👥',
      color: 'border-l-blue-500',
    },
    {
      to: route.adminMemberships(),
      title: i18n.memberships,
      description: i18n.membershipsDesc,
      icon: '💳',
      color: 'border-l-purple-500',
    },
    {
      to: route.adminAttributeCodes(),
      title: i18n.attributeCodes,
      description: i18n.attributeCodesDesc,
      icon: '🔐',
      color: 'border-l-green-500',
    },
    {
      to: route.adminRewards(),
      title: i18n.rewards,
      description: i18n.rewardsDesc,
      icon: '🎁',
      color: 'border-l-yellow-500',
    },
    {
      to: route.adminMigrations(),
      title: i18n.migrations,
      description: i18n.migrationsDesc,
      icon: '🧩',
      color: 'border-l-teal-500',
    },
    {
      to: route.adminPayments(),
      title: i18n.payments,
      description: i18n.paymentsDesc,
      icon: '💰',
      color: 'border-l-orange-500',
    },
  ];

  return (
    <div className="w-full max-w-desktop mx-auto p-6">
      <div className="mb-8">
        <h1 className="text-3xl font-bold mb-2">{i18n.title}</h1>
        <div className="h-1 w-20 bg-linear-to-r from-blue-500 to-purple-500 rounded-full"></div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {adminCards.map((card) => (
          <NavLink
            key={card.to}
            to={card.to}
            className={`group block p-6 bg-white dark:bg-gray-800 rounded-lg shadow-md hover:shadow-xl transition-all duration-200 border-l-4 ${card.color} border-t border-r border-b border-gray-200 dark:border-gray-700 hover:scale-105`}
          >
            <div className="flex items-start gap-4">
              <div className="text-4xl opacity-80 group-hover:opacity-100 transition-opacity">
                {card.icon}
              </div>
              <div className="flex-1">
                <h2 className="text-xl font-semibold mb-2 group-hover:text-blue-600 dark:group-hover:text-blue-400 transition-colors">
                  {card.title}
                </h2>
                <p className="text-gray-600 dark:text-gray-400 text-sm leading-relaxed">
                  {card.description}
                </p>
              </div>
            </div>
          </NavLink>
        ))}
      </div>
    </div>
  );
}
