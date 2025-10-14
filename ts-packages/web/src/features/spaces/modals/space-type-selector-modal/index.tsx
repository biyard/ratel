import SpaceTypeItem from '@/features/spaces/components/space-type-item';
import { useSpaceTypeSelectModalController } from './use-space-type-selector-modal-controller';

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
            <button
              type="button"
              onClick={ctrl.handleClose}
              className="px-10 text-base font-bold bg-transparent transition-colors hover:text-white min-w-[50px] py-[14.5px] text-neutral-400"
            >
              Cancel
            </button>
            <button
              onClick={ctrl.handleNext}
              disabled={ctrl.isLoading.get()}
              className={`w-full py-[14.5px] font-bold text-base rounded-[10px] ${
                !ctrl.isLoading.get()
                  ? 'bg-primary text-black hover:bg-primary/80'
                  : 'bg-disabled-button-bg text-disabled-button-text cursor-not-allowed'
              } transition-colors`}
            >
              {ctrl.isLoading.get() ? 'Sending...' : 'Send'}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
