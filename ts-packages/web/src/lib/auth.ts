// 이 모듈은 서버 전용 코드를 포함하고 있음을 명시합니다.
'use server';

import { config } from '@/config';

// Commenting out cookies() since it's a Next.js-specific API not compatible with React Router
/**
 * Simple check to see if user is logged in based on cookies.
 * @returns {boolean} true: logged in, false: not logged in
 */
// export async function isLoggedIn(): Promise<boolean> {
//   const cookieStore = await cookies();
//
//   const token = cookieStore.get(`${config.env}_auth_token`)?.value;
//   const userId = cookieStore.get(`${config.env}_sid`)?.value;
//
//   const isLoggedIn = !!token || !!userId;
//
//   return isLoggedIn;
// }
