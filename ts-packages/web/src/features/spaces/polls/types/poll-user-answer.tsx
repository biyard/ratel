import { SurveyAnswer } from './poll-question';

export interface PollUserAnswer {
  pk: string;
  sk: string;
  created_at: number;
  answers: SurveyAnswer[];
  respondent?: RespondentAttr;
  user_pk?: string;
  display_name?: string;
  profile_url?: string;
  username?: string;
}

export interface RespondentAttr {
  gender?: 'Male' | 'Female';
  age?: {
    type: 'Specific' | 'Range';
    value?: number;
    inclusive_min?: number;
    inclusive_max?: number;
  };
  school?: string;
}
