import { Answer } from '@/lib/api/models/response';
import { Question, SurveyCreateRequest } from '@/lib/api/models/survey';

export interface Poll {
  surveys: SurveyCreateRequest[];
}

export interface MappedResponse {
  question: Question;
  answers: Answer[];
}
