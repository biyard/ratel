import SpaceTypeItem from '@/features/spaces/components/space-type-item';
import { useSpaceTypeSelectModalController } from './use-space-type-selector-modal-controller';
import { Button } from '@/components/ui/button';
import { useTranslation } from 'react-i18next';

export const i18nSpaceTypeSelectModal = {
  en: {
    btn_create: 'Create',
  },
  ko: {
    btn_create: '생성',
  },
};

export default function SpaceCreateModal({ feed_id }: { feed_id: string }) {
  const ctrl = useSpaceTypeSelectModalController(feed_id);
  const { t } = useTranslation('SpaceTypeSelectModal');

  const renderedForms = ctrl.spaceDefinitions.map((form, i) => (
    <SpaceTypeItem
      key={form.labelKey}
      spaceDefinition={form}
      selected={i === ctrl.selected.get()}
      onClick={() => ctrl.handleSelect(i)}
    />
  ));

  return (
    <div className="w-full max-w-[600px]">
      <div className="w-full">
        <div className="flex flex-col gap-2.5 p-1.5">
          <div className="flex overflow-y-auto flex-col gap-2.5 p-1.5 w-full max-mobile:h-[350px]">
            {renderedForms}
          </div>

          <div className="flex flex-row gap-2.5">
            <Button variant="text" onClick={ctrl.handleClose}>
              Cancel
            </Button>
            <Button
              variant="primary"
              onClick={ctrl.handleNext}
              disabled={ctrl.isLoading.get()}
              className="flex-1"
            >
              {ctrl.isLoading.get() ? 'Sending...' : t('btn_create')}
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}
