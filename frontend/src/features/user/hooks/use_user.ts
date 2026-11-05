import { useMutation } from '@tanstack/react-query';

import { postAdminLogin, postUserRegisterNewsletter } from '@user/api/user_api';
import type {
  AdminLoginRequest,
  UserNewsletterRegisterRequest,
} from '@user/user_types';

export const useUserNewsletterRegister = () => {
  return useMutation({
    mutationFn: (payload: UserNewsletterRegisterRequest) =>
      postUserRegisterNewsletter(payload),
  });
};

export const useAdminLogin = () => {
  return useMutation({
    mutationFn: (payload: AdminLoginRequest) => postAdminLogin(payload),
  });
};
