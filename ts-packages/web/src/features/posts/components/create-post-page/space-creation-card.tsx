import { Input } from '@/components/ui/input';
import type { I18nCreatePostPage } from './i18n';

interface SpaceCreationCardProps {
  spaceName: string;
  spaceDescription: string;
  onSpaceNameChange: (value: string) => void;
  onSpaceDescriptionChange: (value: string) => void;
  t: I18nCreatePostPage;
}

export function SpaceCreationCard({
  spaceName,
  spaceDescription,
  onSpaceNameChange,
  onSpaceDescriptionChange,
  t,
}: SpaceCreationCardProps) {
  return (
    <div className="mb-5 p-5 border border-input-box-border bg-input-box-bg rounded-md">
      <h3 className="text-text-primary text-lg font-semibold mb-4">
        {t.create_space_title}
      </h3>
      <div className="space-y-4">
        <div>
          <Input
            type="text"
            placeholder={t.space_name_placeholder}
            value={spaceName}
            onChange={(e) => onSpaceNameChange(e.target.value)}
            className="w-full text-text-primary bg-background border-input-box-border"
          />
        </div>
        <div>
          <Input
            type="text"
            placeholder={t.space_description_placeholder}
            value={spaceDescription}
            onChange={(e) => onSpaceDescriptionChange(e.target.value)}
            className="w-full text-text-primary bg-background border-input-box-border"
          />
        </div>
      </div>
    </div>
  );
}
