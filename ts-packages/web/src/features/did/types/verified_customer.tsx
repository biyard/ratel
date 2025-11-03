import { VerifiedGender } from './verified_gender';

export interface VerifiedCustomer {
  birthDate: string;
  gender: VerifiedGender;
  id: string;
  isForeigner: boolean;
  name: string;
  phoneNumber: string;
}
