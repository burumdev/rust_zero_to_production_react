import type { UserNewsletterRegisterRequest } from '@user/user_types';
import { apiFetch } from '@utils/api_client';

export type UserNewsletterRegisterResponse = {
  success: boolean;
  message: string;
};

export const postUserRegisterNewsletter = (
  data: UserNewsletterRegisterRequest,
) => {
  return apiFetch<UserNewsletterRegisterResponse>('/subscriptions', {
    method: 'POST',
    body: JSON.stringify(data),
  });
};
