import { useTranslation } from 'react-i18next';

export default function DropdownMenu({
  onclose,
  ondelete,
}: {
  onclose: () => void;
  ondelete: () => void;
}) {
  const { t } = useTranslation('Space');

  const menuItems = [
    // {
    //   label: t('see_committee_list'),
    //   disabled: true,
    //   action: () => {},
    // },
    // {
    //   label: t('change_category'),
    //   disabled: true,
    //   action: () => {},
    // },
    {
      label: t('delete'),
      disabled: false,
      action: () => {
        ondelete();
      },
    },
  ];

  const handleKeyDown = (
    e: React.KeyboardEvent,
    action: () => void,
    disabled?: boolean,
  ) => {
    if (disabled) return;

    switch (e.key) {
      case 'Enter':
      case ' ':
        e.preventDefault();
        action();
        onclose();
        break;
      case 'Escape':
        e.preventDefault();
        onclose();
        break;
      case 'ArrowDown':
        e.preventDefault();
        const nextItem = e.currentTarget.nextElementSibling as HTMLElement;
        nextItem?.focus();
        break;
      case 'ArrowUp':
        e.preventDefault();
        const prevItem = e.currentTarget.previousElementSibling as HTMLElement;
        prevItem?.focus();
        break;
      case 'Home':
        e.preventDefault();
        const menu = e.currentTarget.closest('[role="menu"]');
        const firstItem = menu?.querySelector(
          '[role="menuitem"]:not([aria-disabled="true"])',
        ) as HTMLElement;
        firstItem?.focus();
        break;
      case 'End':
        e.preventDefault();
        const menuEnd = e.currentTarget.closest('[role="menu"]');
        const items = menuEnd?.querySelectorAll(
          '[role="menuitem"]:not([aria-disabled="true"])',
        );
        const lastItem = items?.[items.length - 1] as HTMLElement;
        lastItem?.focus();
        break;
    }
  };

  return (
    <div className="w-56 bg-neutral-800 light:bg-white text-text-primary rounded-lg shadow-lg py-2  space-y-1">
      {menuItems.map((item, index) => (
        <div
          key={index}
          role="menuitem"
          tabIndex={item.disabled ? -1 : 0}
          onClick={
            !item.disabled
              ? () => {
                  item.action();
                  onclose();
                }
              : undefined
          }
          onKeyDown={(e) => handleKeyDown(e, item.action, item.disabled)}
          className={`px-4 py-2 hover:bg-hover rounded-md text-text-primary text-sm font-semibold ${
            item.disabled ? 'cursor-not-allowed' : 'cursor-pointer'
          }`}
          aria-disabled={item.disabled}
        >
          {item.label}
        </div>
      ))}
    </div>
  );
}
