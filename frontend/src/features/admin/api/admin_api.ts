import type {
  AdminChangePasswordRequest,
  AdminLoginRequest,
} from '@admin/admin_types';
import { apiFetch } from '@utils/api_client';
import type { MutationResponse } from 'src/common_types';

export const postAdminLogin = async (data: AdminLoginRequest) => {
  return apiFetch<MutationResponse>('login', {
    method: 'POST',
    body: JSON.stringify(data),
  });
};

export const postAdminLogout = async () => {
  return apiFetch('admin/logout', {
    method: 'POST',
  });
};

export const postAdminChangePassword = async (
  data: AdminChangePasswordRequest,
) => {
  return apiFetch<MutationResponse>('admin/password', {
    method: 'POST',
    body: JSON.stringify(data),
  });
};
