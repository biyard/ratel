import ChevronRight from '@/assets/icons/chevron-right.svg';
import ThemeModal from '../modal/theme-modal';
import { usePopup } from '@/lib/contexts/popup-service';
import React, { useEffect, useState } from 'react';
import { useTheme } from 'next-themes';

type Theme = 'light' | 'dark' | 'system';

export default function MySettings() {
  const popup = usePopup();
  const { theme, setTheme } = useTheme();
  const [mounted, setMounted] = useState(false);

  // Ensure component is mounted to avoid hydration mismatch
  useEffect(() => {
    setMounted(true);
  }, []);

  const themeLabels: Record<Theme, string> = {
    dark: 'Dark Theme',
    light: 'Light Theme',
    system: 'System Default',
  };

  // Prevent rendering theme text until component is mounted
  const currentTheme = mounted ? (theme as Theme) || 'system' : 'system';

  const handleSaveClick = () => {
    popup
      .open(
        <ThemeModal
          initialTheme={currentTheme as Theme}
          onSave={(selected: Theme) => {
            setTheme(selected);
            popup.close();
          }}
          onCancel={() => popup.close()}
        />,
      )
      .withoutBackdropClose();
  };

  return (
    <div className="w-full max-w-[800px] mx-auto flex flex-col gap-6 px-4 md:px-0">
      {/* Billing Section */}
      <section></section>

      {/* Security Section */}
      <section></section>

      {/* Appearance section */}
      <section className="bg-component-bg p-4 md:p-6 rounded-lg">
        <h2 className="text-lg font-bold mb-4 text-sm text-white">
          Appearance
        </h2>

        <div className="flex flex-col gap-4">
          <SpecBox
            left_text="Theme"
            action_text={themeLabels[currentTheme as Theme]}
            onClick={handleSaveClick}
          />

          <SpecBox left_text="Language" action_text="English" />
        </div>
      </section>
    </div>
  );
}

function SpecBox({
  left_text,
  action_text,
  onClick,
}: {
  left_text: string;
  action_text?: string;
  onClick?: () => void;
}) {
  return (
    <div className="flex items-center justify-between border border-neutral-800 px-4 py-8 rounded-md">
      <p className="text-lg font-bold  text-sm text-white">{left_text}</p>

      {/* button action */}
      <button
        className="flex items-center gap-2 text-primary cursor-pointer"
        onClick={onClick}
      >
        {action_text}
        <ChevronRight className="w-4 h-4" />
      </button>
    </div>
  );
}
