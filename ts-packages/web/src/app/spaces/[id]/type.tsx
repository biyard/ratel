import { Question, SurveyCreateRequest } from '@/lib/api/models/survey';
import { Answer } from '@/lib/api/models/response';

export interface Poll {
  surveys: SurveyCreateRequest[];
}

export interface SurveyAnswer {
  answers: Answer[];
  is_completed: boolean;
}

export interface MappedResponse {
  question: Question;
  answers: Answer[];
}
