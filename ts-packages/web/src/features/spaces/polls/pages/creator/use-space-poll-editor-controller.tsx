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

export class SpacePollEditorController {
  constructor(
    public space: Space,
    public poll: Poll,
    public questions: State<PollQuestion[]>,
    public t: TFunction<'SpaceSurvey', undefined>,
    public editing: State<boolean>,
    public answers: State<Record<number, SurveyAnswer>>,
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
  };

  handleDiscard = () => {
    this.editing.set(false);
  };

  handleUpdateAnswer = (questionIdx: number, answer: SurveyAnswer) => {
    logger.debug(
      `handleUpdateAnswer called for questionIdx ${questionIdx}`,
      answer,
    );
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

  return new SpacePollEditorController(
    space,
    poll,
    new State(questions),
    t,
    new State(editing),
    new State(answers),
  );
}
