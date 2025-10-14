import SpaceTypeItem from '@/features/spaces/components/space-type-item';
import { useSpaceTypeSelectModalController } from './use-space-type-selector-modal-controller';
import { Button } from '@/components/ui/button';

export default function SpaceCreateModal({ feed_id }: { feed_id: string }) {
  const ctrl = useSpaceTypeSelectModalController(feed_id);

  const renderedForms = ctrl.spaceDefinitions.map((form, i) => (
    <SpaceTypeItem
      key={form.labelKey}
      spaceDefinition={form}
      selected={i === ctrl.selected.get()}
      onClick={() => ctrl.handleSelect(i)}
    />
  ));

  return (
    <div className="w-full max-w-[95vw] max-tablet:w-fit">
      <div className="mobile:w-[400px] max-mobile:w-full">
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
              {ctrl.isLoading.get() ? 'Sending...' : 'Send'}
            </Button>
          </div>
        </div>
      </div>
    </div>
  );
}
