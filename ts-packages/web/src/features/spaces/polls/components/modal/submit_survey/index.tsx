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
        <label className="text-[15px]/[28px] font-semibold text-modal-label-text whitespace-pre-line">
          {ctrl.t('modal_desc')}
        </label>

        <div className="flex flex-row w-full justify-end gap-3 mt-10">
          <Button onClick={ctrl.handleClose}>{ctrl.t('cancel')}</Button>
          <Button variant="primary" onClick={ctrl.handleSubmit}>
            {ctrl.t('save')}
          </Button>
        </div>
      </div>
    </div>
  );
}
