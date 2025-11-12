import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import { TFunction } from 'i18next';
import usePollSpace from '../../hooks/use-poll-space';
import { useTranslation } from 'react-i18next';
import { Poll } from '../../types/poll';
import { SurveyAnswer } from '../../types/poll-question';
import { logger } from '@/lib/logger';
import { State } from '@/types/state';
import { useEffect, useState } from 'react';
import { useUserInfo } from '@/hooks/use-user-info';
import { LoginModal } from '@/components/popup/login-popup';
import { usePopup } from '@/lib/contexts/popup-service';
import { UserResponse } from '@/lib/api/ratel/users.v3';
import { usePollResponseMutation } from '../../hooks/use-poll-response-mutation';
import { showErrorToast, showSuccessToast } from '@/lib/toast';
import { route } from '@/route';
import { NavigateFunction, useNavigate } from 'react-router';
import SubmitSurveyPopup from '../../components/modal/submit_survey';

export class SpacePollViewerController {
  constructor(
    public space: Space,
    public poll: Poll,
    public navigate: NavigateFunction,
    public t: TFunction<'SpaceSurvey', undefined>,
    public answers: State<Record<number, SurveyAnswer>>,
    public user: UserResponse | null,
    public popup: ReturnType<typeof usePopup>,
    public submitPollResponse: ReturnType<typeof usePollResponseMutation>,
  ) {}

  handleUpdateAnswer = (questionIdx: number, answer: SurveyAnswer) => {
    logger.debug(
      `handleUpdateAnswer called for questionIdx ${questionIdx}`,
      answer,
    );

    const currentAnswers = this.answers.get();
    currentAnswers[questionIdx] = answer;
    this.answers.set({ ...currentAnswers });
  };

  handleBack = () => {
    this.navigate(route.spacePolls(this.space.pk));
  };

  handleSubmit = () => {
    const current = this.answers.get();
    const total = this.poll.questions.length;

    const defaultAnswerByType = (
      answer_type: SurveyAnswer['answer_type'],
    ): SurveyAnswer => {
      switch (answer_type) {
        case 'single_choice':
        case 'dropdown':
        case 'linear_scale':
        case 'short_answer':
        case 'subjective':
          return { answer_type, answer: null };
        case 'multiple_choice':
        case 'checkbox':
          return { answer_type, answer: [] };
        default:
          return { answer_type, answer: null };
      }
    };

    const payload: SurveyAnswer[] = Array.from({ length: total }, (_, i) => {
      const existing = current[i];
      if (existing !== undefined) return existing;

      const q = this.poll.questions[i];
      return defaultAnswerByType(q.answer_type as SurveyAnswer['answer_type']);
    });

    if (this.poll.response_editable) {
      this.submitPollResponse.mutate(
        {
          spacePk: this.space.pk,
          pollSk: this.poll.sk,
          answers: payload,
        },
        {
          onSuccess: () => {
            showSuccessToast(this.t('success_submit_answer'));
          },
          onError: (err) => {
            logger.error('submit answer failed: ', err);
            showErrorToast(this.t('failed_submit_answer'));
          },
        },
      );
    } else {
      this.popup
        .open(
          <SubmitSurveyPopup
            spacePk={this.space.pk}
            pollSk={this.poll.sk}
            answers={payload}
          />,
        )
        .withTitle(this.t('modal_title'));
    }
  };

  handleLogin = () => {
    this.popup
      .open(<LoginModal />)
      .withTitle(this.t('Nav:join_the_movement'))
      .withoutBackdropClose();
  };
}

export function useSpacePollViewerController(spacePk, pollPk) {
  // Fetching data from remote
  const { data: space } = useSpaceById(spacePk);
  const { data: poll } = usePollSpace(spacePk, pollPk);
  const { data: user } = useUserInfo();
  const navigator = useNavigate();

  // mutations
  const usePollResponse = usePollResponseMutation();

  const { t } = useTranslation('SpaceSurvey');
  const popup = usePopup();
  const answers = useState<Record<number, SurveyAnswer>>(
    poll?.myResponse.reduce(
      (acc, answer, idx) => {
        acc[idx] = answer;
        return acc;
      },
      {} as Record<number, SurveyAnswer>,
    ) || {},
  );

  useEffect(() => {
    answers[1](
      poll?.myResponse.reduce(
        (acc, answer, idx) => {
          acc[idx] = answer;
          return acc;
        },
        {} as Record<number, SurveyAnswer>,
      ) || {},
    );
  }, [poll?.myResponse]);

  return new SpacePollViewerController(
    space,
    poll,
    navigator,
    t,
    new State(answers),
    user,
    popup,
    usePollResponse,
  );
}
