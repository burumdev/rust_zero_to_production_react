import { useMutation } from '@tanstack/react-query';

import { postUserRegisterNewsletter } from '@user/api/user_api';
import type { UserNewsletterRegisterRequest } from '@user/user_types';

export const useUserNewsletterRegister = () => {
  return useMutation({
    mutationFn: (payload: UserNewsletterRegisterRequest) =>
      postUserRegisterNewsletter(payload),
  });
};
