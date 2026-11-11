import Button from '@components/Button';
import TextInput from '@components/TextInput';
import { useAdminLogin } from '@admin/hooks/use_admin';
import type { AdminLoginRequest } from '@admin/admin_types';
import { useEffect, useState } from 'react';

type FormField = 'username' | 'password';
type FormRecord = Record<FormField, string>;

const Login = () => {
  const {
    mutate: sendForm,
    data: successData,
    error: errorData,
    isPending,
    isError,
    isSuccess,
  } = useAdminLogin();

  const initialFormData: Record<FormField, string> = {
    username: '',
    password: '',
  };

  const [formData, setFormData] = useState<FormRecord>(initialFormData);
  const [formDataErrors, setFormDataErrors] =
    useState<FormRecord>(initialFormData);
  const [isFormSubmitButtonDisabled, setFormSubmitButtonDisabled] =
    useState(true);

  const resetFormDataError = (field: FormField | 'all') => {
    if (field === 'all') {
      setFormDataErrors(initialFormData);
    } else {
      setFormDataErrors({
        ...formDataErrors,
        [field]: '',
      });
    }
  };

  const isFormValid = () => {
    return (
      Object.values(formDataErrors).every((item) => item === '') &&
      Object.values(formData).every((item) => item !== '')
    );
  };

  const handleFormDataChange = (field: FormField, value: string) => {
    if (value === '') {
      setFormDataErrors({
        ...formDataErrors,
        [field]: 'This field can not be empty.',
      });
    } else {
      switch (field) {
        case 'username':
          if (value.length < 5) {
            setFormDataErrors({
              ...formDataErrors,
              [field]: 'Username must be at least 5 characters.',
            });
          } else {
            resetFormDataError(field);
          }
          break;
        case 'password':
          if (value.length < 10) {
            setFormDataErrors({
              ...formDataErrors,
              [field]: 'Password must be at least 10 characters.',
            });
          } else {
            resetFormDataError(field);
          }
          break;
        default:
          break;
      }
    }

    setFormData({
      ...formData,
      [field]: value,
    });
  };

  useEffect(() => {
    setFormSubmitButtonDisabled(!isFormValid());
  }, [formDataErrors]);

  const handleSubmit = () => {
    if (isFormValid()) {
      sendForm(formData as AdminLoginRequest);
    }
  };

  return (
    <div className="w-[600px] text-darkgray">
      <h1 className="text-center text-lg text-white">Login</h1>
      <div className="mt-8 h-forminput">
        <TextInput
          placeholder="Username*"
          isFocused
          onChange={(e: any) =>
            handleFormDataChange('username', e.target.value)
          }
          value={formData.username}
          errorStr={formDataErrors['username']}
        />
      </div>
      <div className="mt-8 h-forminput">
        <TextInput
          placeholder="Password*"
          onChange={(e: any) =>
            handleFormDataChange('password', e.target.value)
          }
          value={formData.password}
          errorStr={formDataErrors['password']}
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
          text="LOGIN"
          backgroundClass="bg-highlight"
          dropShadowColor="highlight-darker"
          isDisabled={isFormSubmitButtonDisabled || isPending}
          isLoading={isPending}
        />
      </div>
    </div>
  );
};

export default Login;
