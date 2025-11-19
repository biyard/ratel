import { Button } from '@/components/ui/button';
import { useCompleteSurveyModalController } from './use-complete-survey-modal-controller';

export type CompleteSurveyPopupProps = {
  onConfirm: () => void;
};

export default function CompleteSurveyPopup({
  onConfirm,
}: CompleteSurveyPopupProps) {
  const ctrl = useCompleteSurveyModalController();

  return (
    <div className="flex flex-col w-[450px] max-w-[450px] max-tablet:w-full! max-tablet:max-w-full! gap-5">
      <div className="flex flex-col py-2.5 gap-[5px]">
        <label className="text-[15px]/[28px] font-semibold text-modal-label-text whitespace-pre-line">
          {ctrl.t('modal_desc')}
        </label>

        <div className="flex flex-row w-full justify-end gap-3 mt-10">
          <Button
            variant="primary"
            onClick={onConfirm}
            data-testid="complete-survey-modal-btn-confirm"
          >
            {ctrl.t('confirm')}
          </Button>
        </div>
      </div>
    </div>
  );
}
