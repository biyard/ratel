import { useState } from 'react';
/* import { useSpacePollEditorData } from './use-space-poll-editor-data'; */
import { State } from '@/types/state';
import usePollSpace from '../../hooks/use-poll-space';
import { Poll } from '../../types/poll';
import { useSpaceById } from '@/features/spaces/hooks/use-space-by-id';
import { Space } from '@/features/spaces/types/space';
import { logger } from '@/lib/logger';
import { createDefaultQuestion, PollQuestion } from '../../types/poll-question';
import {
  SurveyAnswer,
  SurveyAnswerType,
} from '@/features/spaces/types/survey-type';
import { TFunction } from 'i18next';
import { useTranslation } from 'react-i18next';
import { useUpdateTimeRangeMutation } from '../../hooks/use-update-time-range-mutation';
import { useUpdateQuestionsMutation } from '../../hooks/use-update-questions-mutation';
import { useUpdateResponseEditableMutation } from '../../hooks/use-update-response-editable-mutation';
import { showErrorToast, showSuccessToast } from '@/lib/toast';

export class SpacePollEditorController {
  constructor(
    public space: Space,
    public poll: Poll,
    public questions: State<PollQuestion[]>,
    public t: TFunction<'SpaceSurvey', undefined>,
    public editing: State<boolean>,
    public answers: State<Record<number, SurveyAnswer>>,
    public updateTimeRange: ReturnType<typeof useUpdateTimeRangeMutation>,
    public updateQuestions: ReturnType<typeof useUpdateQuestionsMutation>,
    public updateResponseEditable: ReturnType<
      typeof useUpdateResponseEditableMutation
    >,
  ) {}

  handleAddQuestion = () => {
    this.questions.set([
      ...this.questions.get(),
      createDefaultQuestion(SurveyAnswerType.SingleChoice),
    ]);
  };

  handleUpdateQuestion = (index: number, newOne: PollQuestion) => {
    logger.debug(`handleUpdateQuestion called for index ${index}`, newOne);
    const questions = this.questions.get();
    questions[index] = newOne;

    this.questions.set([...questions]);
  };

  handleRemoveQuestion = (index: number) => {
    const newQuestions = this.questions.get().filter((_, i) => i !== index);
    this.questions.set(newQuestions);
  };

  handleEdit = () => {
    this.editing.set(true);
  };

  handleSave = () => {
    this.editing.set(false);

    try {
      this.updateQuestions.mutate({
        spacePk: this.space.pk,
        pollSk: this.poll.sk,
        questions: this.questions.get(),
      });

      showSuccessToast(this.t('success_change_response'));
    } catch (err) {
      logger.error('save failed: ', err);
      showErrorToast(this.t('failed_change_response'));
    }
  };

  handleDiscard = () => {
    this.editing.set(false);
  };

  handleUpdateAnswer = (questionIdx: number, answer: SurveyAnswer) => {
    logger.debug(
      `handleUpdateAnswer called for questionIdx ${questionIdx}`,
      answer,
    );
    const currentAnswers = this.answers.get();
    currentAnswers[questionIdx] = answer;
    this.answers.set({ ...currentAnswers });
  };

  onChangeTimeRange = (started_at: number, ended_at: number) => {
    logger.debug(
      `onChangeTimeRange called: start=${started_at}, end=${ended_at}`,
    );

    try {
      this.updateTimeRange.mutate({
        spacePk: this.space.pk,
        pollSk: this.poll.sk,
        started_at,
        ended_at,
      });

      showSuccessToast(this.t('success_update_time'));
    } catch (err) {
      logger.error('update time range failed: ', err);
      showErrorToast(this.t('failed_update_time'));
    }
  };

  onChangeResponseEditable = (response_editable: boolean) => {
    logger.debug(
      `onChangeResponseEditable called: response_editable=${response_editable}`,
    );

    try {
      this.updateResponseEditable.mutate({
        spacePk: this.space.pk,
        pollSk: this.poll.sk,
        response_editable,
      });

      showSuccessToast(this.t('success_change_response'));
    } catch (err) {
      logger.error('change response failed: ', err);
      showErrorToast(this.t('failed_change_response'));
    }
  };
}

export function useSpacePollEditorController(spacePk: string, pollPk: string) {
  const { data: space } = useSpaceById(spacePk);
  const { data: poll } = usePollSpace(spacePk, pollPk);
  const questions = useState(poll.questions || []);
  const { t } = useTranslation('SpaceSurvey');
  const editing = useState(false);
  // FIXME: This should be my current answers
  const answers = useState<Record<number, SurveyAnswer>>({});

  const updateTimeRange = useUpdateTimeRangeMutation();
  const updateQuestions = useUpdateQuestionsMutation();
  const updateResponseEditable = useUpdateResponseEditableMutation();

  return new SpacePollEditorController(
    space,
    poll,
    new State(questions),
    t,
    new State(editing),
    new State(answers),
    updateTimeRange,
    updateQuestions,
    updateResponseEditable,
  );
}
