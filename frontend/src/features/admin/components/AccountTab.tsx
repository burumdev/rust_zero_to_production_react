import type { AdminChangePasswordRequest } from '@admin/admin_types';
import { useAdminChangePassword } from '@admin/hooks/use_admin';
import Button from '@components/Button';
import TextInput from '@components/TextInput';
import useFormValidator, {
  type ValidationRule,
  type ValidationRuleSet,
} from '@hooks/useFormValidator';
import { useEffect, useState } from 'react';

type FormData = AdminChangePasswordRequest;

const INITIAL_FORM_DATA: FormData = {
  current_password: '',
  new_password: '',
  new_password_check: '',
};

const PASSWORD_RULE: ValidationRule<FormData> = {
  ruleName: 'min',
  min: 10,
  errorStr: 'Min password is 10 characters',
};

const FORM_VALIDATION_RULES: ValidationRuleSet<FormData> = {
  current_password: [PASSWORD_RULE],
  new_password: [
    PASSWORD_RULE,
    {
      ruleName: 'notSame',
      samenessKey: 'current_password',
      errorStr: 'Same as current password',
    },
  ],
  new_password_check: [
    PASSWORD_RULE,
    {
      ruleName: 'same',
      samenessKey: 'new_password',
      errorStr: 'Passwords should match',
    },
  ],
};

const Account = () => {
  const [isFormSubmitButtonDisabled, setFormSubmitButtonDisabled] =
    useState(true);
  const { formData, formErrors, setFormField, isFormValid, resetForm } =
    useFormValidator<FormData>(INITIAL_FORM_DATA, FORM_VALIDATION_RULES);

  const {
    data: successData,
    mutate: sendForm,
    error: errorData,
    isPending,
    isError,
    isSuccess,
  } = useAdminChangePassword();

  const handleSubmit = () => {
    if (isFormValid()) {
      sendForm(formData as AdminChangePasswordRequest);
    }
  };

  useEffect(() => {
    if (isSuccess) {
      resetForm();
    }
  }, [isSuccess]);

  useEffect(() => {
    setFormSubmitButtonDisabled(!isFormValid());
  }, [formData, isFormValid]);

  return (
    <div className="w-full h-full mt-4 px-4 grid grid-cols-2 gap-4">
      <div className="p-4 step-box bg-darkgray">
        <h1>Change Password</h1>
        <p className="mt-3 text-sm">
          Change your password to prevent it falling into wrong hands and legs
        </p>
        <div className="text-darkgray">
          <div className="mt-6 h-forminput">
            <TextInput
              placeholder="Current Password*"
              isFocused
              onChange={(e: any) =>
                setFormField('current_password', e.target.value)
              }
              value={formData.current_password}
              // Normally form validator supports multiple error messages
              // But for UX and UI purposes we pluck the first error
              errorStr={formErrors['current_password']?.[0].errorStr}
              type="password"
            />
          </div>
          <div className="mt-6 h-forminput">
            <TextInput
              placeholder="New Password*"
              onChange={(e: any) =>
                setFormField('new_password', e.target.value)
              }
              value={formData.new_password}
              errorStr={formErrors['new_password']?.[0].errorStr}
              type="password"
            />
          </div>
          <div className="mt-6 h-forminput">
            <TextInput
              placeholder="New Password Again*"
              onChange={(e: any) =>
                setFormField('new_password_check', e.target.value)
              }
              value={formData.new_password_check}
              errorStr={formErrors['new_password_check']?.[0].errorStr}
              type="password"
            />
          </div>
          <div
            className={`h-forminput flex items-end mt-2 text-xs ${isSuccess ? 'text-success' : 'text-error'}`}
          >
            {isSuccess && successData.message}
            {isError && errorData.message}
          </div>
          <div className="mt-4 h-forminput flex justify-end">
            <Button
              onClick={handleSubmit}
              text="CHANGE"
              backgroundClass="bg-highlight"
              dropShadowColor="highlight-darker"
              isDisabled={isFormSubmitButtonDisabled || isPending}
              isLoading={isPending}
            />
          </div>
        </div>
      </div>
    </div>
  );
};

export default Account;
