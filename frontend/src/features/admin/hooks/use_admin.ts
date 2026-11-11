import { useMutation } from '@tanstack/react-query';

import { postAdminLogin } from '@admin/api/admin_api';
import type { AdminLoginRequest } from '@admin/admin_types';

export const useAdminLogin = () => {
  return useMutation({
    mutationFn: (payload: AdminLoginRequest) => postAdminLogin(payload),
  });
};
