import type {
  AdminChangePasswordRequest,
  AdminLoginRequest,
  AdminPublishNewsletterRequest,
} from '@admin/admin_types';
import { apiFetch } from '@utils/api_client';
import type { MutationResponse } from '../../../common_types';

export const postAdminLogin = async (data: AdminLoginRequest) => {
  return apiFetch<MutationResponse>('login', {
    method: 'POST',
    body: JSON.stringify(data),
  });
};

export const postAdminLogout = async (idemKey: string) => {
  return apiFetch(
    'admin/logout',
    {
      method: 'POST',
    },
    idemKey,
  );
};

export const postAdminChangePassword = async (
  data: AdminChangePasswordRequest,
  idemKey: string,
) => {
  return apiFetch<MutationResponse>(
    'admin/password',
    {
      method: 'POST',
      body: JSON.stringify(data),
    },
    idemKey,
  );
};

export const postAdminPublishNewsletter = async (
  data: AdminPublishNewsletterRequest,
  idemKey: string,
) => {
  return apiFetch<MutationResponse>(
    'admin/newsletters',
    {
      method: 'POST',
      body: JSON.stringify(data),
    },
    idemKey,
  );
};
