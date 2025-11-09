import type { Gender } from './attribute-code-response';

export interface CreateAttributeCodeRequest {
  birth_date?: string;
  gender?: Gender;
  university?: string;
}
