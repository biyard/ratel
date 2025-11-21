import { Button } from '@/components/ui/button';
import { SurveyAnswer } from '../../../types/poll-question';
import { useSubmitSurveyModalController } from './use-submit-survey-modal-controller';

export type SubmitSurveyPopupProps = {
  spacePk: string;
  pollSk: string;
  answers: SurveyAnswer[];
};

export default function SubmitSurveyPopup({
  spacePk,
  pollSk,
  answers,
}: SubmitSurveyPopupProps) {
  const ctrl = useSubmitSurveyModalController(spacePk, pollSk, answers);

  return (
    <div className="flex flex-col w-[450px] max-w-[450px] max-tablet:!w-full max-tablet:!max-w-full gap-5">
      <div className="flex flex-col py-2.5 gap-[5px]">
        <label className="font-semibold whitespace-pre-line text-[15px]/[28px] text-modal-label-text">
          {ctrl.t('modal_desc')}
        </label>

        <div className="flex flex-row gap-3 justify-end mt-10 w-full">
          <Button data-testid="btn-cancel" onClick={ctrl.handleClose}>
            {ctrl.t('cancel')}
          </Button>
          <Button
            data-testid="btn-confirm"
            variant="primary"
            onClick={ctrl.handleSubmit}
          >
            {ctrl.t('save')}
          </Button>
        </div>
      </div>
    </div>
  );
}
