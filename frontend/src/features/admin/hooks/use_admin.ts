import { useMutation } from '@tanstack/react-query';

import {
  postAdminChangePassword,
  postAdminLogin,
  postAdminLogout,
} from '@admin/api/admin_api';

import type {
  AdminChangePasswordRequest,
  AdminLoginRequest,
} from '@admin/admin_types';

export const useAdminLogin = () => {
  return useMutation({
    mutationKey: ['adminLogin'],
    mutationFn: (payload: AdminLoginRequest) => postAdminLogin(payload),
  });
};

export const useLoginState = (): boolean => {
  return sessionStorage.getItem('login') === 'yes';
};

export const useAdminLogout = () => {
  return useMutation({
    mutationKey: ['adminLogout'],
    mutationFn: () => postAdminLogout(),
  });
};

export const useAdminChangePassword = () => {
  return useMutation({
    mutationKey: ['adminChangePassword'],
    mutationFn: (payload: AdminChangePasswordRequest) =>
      postAdminChangePassword(payload),
  });
};
