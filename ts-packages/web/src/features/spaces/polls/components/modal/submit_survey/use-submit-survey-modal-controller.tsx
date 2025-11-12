import { usePopup } from '@/lib/contexts/popup-service';
import { SurveyAnswer } from '../../../types/poll-question';
import { usePollResponseMutation } from '../../../hooks/use-poll-response-mutation';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { logger } from '@/lib/logger';
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';

export class SubmitSurveyModalController {
  constructor(
    public t: TFunction<'SpacePollSubmitSurvey', undefined>,
    public popup: ReturnType<typeof usePopup>,
    public submitPollResponse: ReturnType<typeof usePollResponseMutation>,
    public spacePk: string,
    public pollSk: string,
    public answers: SurveyAnswer[],
  ) {}

  handleSubmit = async () => {
    try {
      await this.submitPollResponse.mutateAsync({
        spacePk: this.spacePk,
        pollSk: this.pollSk,
        answers: this.answers,
      });

      showSuccessToast(this.t('success_submit_answer'));
      this.popup.close();
    } catch (err) {
      logger.error('submit answer failed: ', err);
      showErrorToast(this.t('failed_submit_answer'));
      // Don't close popup on error so user can retry
    }
  };

  handleClose = () => {
    this.popup.close();
  };
}

export function useSubmitSurveyModalController(
  spacePk: string,
  pollSk: string,
  answers: SurveyAnswer[],
) {
  const { t } = useTranslation('SpacePollSubmitSurvey');
  const popup = usePopup();
  const usePollResponse = usePollResponseMutation();
  return new SubmitSurveyModalController(
    t,
    popup,
    usePollResponse,
    spacePk,
    pollSk,
    answers,
  );
}
