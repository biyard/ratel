import {
  SurveyAnswer,
  SurveyAnswerType,
  PollQuestion,
  SurveyQuestionWithAnswer,
} from '@/features/spaces/polls/types/poll-question';

export default function validateSurveyAnswer(
  questions: PollQuestion[],
  answers: SurveyAnswer[],
): boolean {
  if (questions.length !== answers.length) return false;

  return questions.every((question, index) => {
    const answer = answers[index];

    if (question.answer_type !== answer.answer_type) return false;

    if (question.is_required) {
      if (
        !validateAnswerHasValue({
          answer_type: question.answer_type,
          question: question,
          answer: answer,
        } as SurveyQuestionWithAnswer)
      )
        return false;
    }

    return true;
  });
}

function validateAnswerHasValue(
  questinonWithAnswer: SurveyQuestionWithAnswer,
): boolean {
  const { answer, answer_type, question } = questinonWithAnswer;
  switch (answer_type) {
    case SurveyAnswerType.SingleChoice:
      return answer.answer !== undefined;
    case SurveyAnswerType.MultipleChoice:
      return Array.isArray(answer.answer) && answer.answer.length > 0;
    case SurveyAnswerType.ShortAnswer:
    case SurveyAnswerType.Subjective:
      return (
        typeof answer.answer === 'string' && answer.answer.trim().length > 0
      );
    case SurveyAnswerType.Checkbox:
      return Array.isArray(answer.answer) && question.is_multi
        ? answer.answer.length > 0
        : answer.answer.length === 1;
    case SurveyAnswerType.Dropdown:
      return answer.answer !== undefined;
    case SurveyAnswerType.LinearScale:
      return answer.answer !== undefined;
    default:
      return false;
  }
}
