import type { AdminLoginRequest } from '@admin/admin_types';
import { apiFetch } from '@utils/api_client';
import type { MutationResponse } from 'src/common_types';

export const postAdminLogin = (data: AdminLoginRequest) => {
  return apiFetch<MutationResponse>('/login', {
    method: 'POST',
    body: JSON.stringify(data),
  });
};
