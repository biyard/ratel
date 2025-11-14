import { usePopup } from '@/lib/contexts/popup-service';
import { useTranslation } from 'react-i18next';
import { TFunction } from 'i18next';

export class CompleteSurveyModalController {
  constructor(
    public t: TFunction<'SpacePollSubmitSurvey', undefined>,
    public popup: ReturnType<typeof usePopup>,
  ) {}

  handleConfirm = () => {
    window.location.reload();
    this.popup.close();
  };

  handleClose = () => {
    this.popup.close();
  };
}

export function useCompleteSurveyModalController() {
  const { t } = useTranslation('SpaceCompleteSurvey');
  const popup = usePopup();
  return new CompleteSurveyModalController(t, popup);
}
