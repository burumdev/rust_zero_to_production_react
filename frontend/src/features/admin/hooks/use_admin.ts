import { useMutation } from '@tanstack/react-query';

import {
  postAdminChangePassword,
  postAdminLogin,
  postAdminLogout,
  postAdminPublishNewsletter,
} from '@admin/api/admin_api';

import type {
  AdminChangePasswordRequest,
  AdminLoginRequest,
  AdminPublishNewsletterRequest,
} from '@admin/admin_types';

import useIdempotencyKey from '../../../hooks/useIdempotencyKey';
import { RETRY_DEFAULT } from '../../../constants';

export const useAdminLogin = () => {
  const idemKey = useIdempotencyKey();

  return useMutation({
    mutationKey: ['adminLogin'],
    mutationFn: (payload: AdminLoginRequest) =>
      postAdminLogin(payload, idemKey.getOrCreateKey()),
    retry: RETRY_DEFAULT,
    onSuccess: () => {
      idemKey.clearKey();
    },
  });
};

export const useLoginState = (): boolean => {
  return sessionStorage.getItem('login') === 'yes';
};

export const useAdminLogout = () => {
  const idemKey = useIdempotencyKey();

  return useMutation({
    mutationKey: ['adminLogout'],
    mutationFn: () => postAdminLogout(idemKey.getOrCreateKey()),
    retry: RETRY_DEFAULT,
    onSuccess: () => {
      idemKey.clearKey();
    },
  });
};

export const useAdminChangePassword = () => {
  const idemKey = useIdempotencyKey();

  return useMutation({
    mutationKey: ['adminChangePassword'],
    mutationFn: (payload: AdminChangePasswordRequest) =>
      postAdminChangePassword(payload, idemKey.getOrCreateKey()),
    retry: RETRY_DEFAULT,
    onSuccess: () => {
      idemKey.clearKey();
    },
  });
};

export const useAdminPublishNewsletter = () => {
  const idemKey = useIdempotencyKey();

  return useMutation({
    mutationKey: ['adminPublishNewsletter'],
    mutationFn: (payload: AdminPublishNewsletterRequest) =>
      postAdminPublishNewsletter(payload, idemKey.getOrCreateKey()),
    retry: RETRY_DEFAULT,
    onSuccess: () => {
      idemKey.clearKey();
    },
  });
};
