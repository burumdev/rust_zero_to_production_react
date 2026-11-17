export type AdminLoginRequest = {
  username: string;
  password: string;
};

export type AdminChangePasswordRequest = {
  current_password: string;
  new_password: string;
  new_password_check: string;
};
