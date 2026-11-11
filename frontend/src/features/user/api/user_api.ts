import type { UserNewsletterRegisterRequest } from '@user/user_types';
import { apiFetch } from '@utils/api_client';
import type { MutationResponse } from 'src/common_types';

export const postUserRegisterNewsletter = (
  data: UserNewsletterRegisterRequest,
) => {
  return apiFetch<MutationResponse>('/subscriptions', {
    method: 'POST',
    body: JSON.stringify(data),
  });
};
