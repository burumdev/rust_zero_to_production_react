import Button from '@components/Button';
import TextInput from '@components/TextInput';

import { useUserNewsletterRegister } from '@user/hooks/use_user';
import type { UserNewsletterRegisterRequest } from '@user/user_types';
import { is_valid_email } from '@utils/validators';
import { useEffect, useState } from 'react';

type FormField = 'email' | 'name';
type FormRecord = Record<FormField, string>;

const NewsletterModal = () => {
  const newsletterRegister = useUserNewsletterRegister();

  const initialFormData: Record<FormField, string> = {
    email: '',
    name: '',
  };
  const [formData, setFormData] = useState<FormRecord>(initialFormData);
  const [formDataErrors, setFormDataErrors] =
    useState<FormRecord>(initialFormData);
  const [isFormSubmitButtonDisabled, setFormSubmitButtonDisabled] =
    useState(true);
  const [isFormSubmitButtonLoading, setFormSubmitButtonLoading] =
    useState(false);

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
        case 'email':
          if (!is_valid_email(value)) {
            setFormDataErrors({
              ...formDataErrors,
              [field]: 'E-mail must be valid.',
            });
          } else {
            resetFormDataError(field);
          }
          break;
        case 'name':
          if (value.length < 5) {
            setFormDataErrors({
              ...formDataErrors,
              name: 'Name must be at least 5 characters.',
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
    setFormSubmitButtonLoading(true);
    newsletterRegister.mutate(formData as UserNewsletterRegisterRequest, {
      onSuccess: () => console.log('TODO: User registration success.'),
      onError: (e) =>
        console.error(`TODO: Handle form submit error. Error: ${e}`),
      onSettled: () => {
        setFormSubmitButtonLoading(false);
        resetFormDataError('all');
      },
    });
  };

  return (
    <div className="max-w-lg">
      <h1 className="text-center text-lg">WELCOME TO ZERO2PROD!</h1>
      <p className="mt-3">
        Enter your email and name to join our newsletter and gain access to our
        exclusive chat rooms.
      </p>
      <div className="mt-4 h-forminput">
        <TextInput
          placeholder="E-mail*"
          isFocused
          onChange={(e: any) => handleFormDataChange('email', e.target.value)}
          value={formData.email}
          errorStr={formDataErrors['email']}
        />
      </div>
      <div className="mt-4 h-forminput">
        <TextInput
          placeholder="Name*"
          onChange={(e: any) => handleFormDataChange('name', e.target.value)}
          value={formData.name}
          errorStr={formDataErrors['name']}
        />
      </div>
      <div className="mt-4 h-forminput flex justify-end">
        <Button
          onClick={handleSubmit}
          text="JOIN"
          backgroundClass="bg-highlight"
          dropShadowColor="highlight-darker"
          isDisabled={isFormSubmitButtonDisabled}
          isLoading={isFormSubmitButtonLoading}
        />
      </div>
    </div>
  );
};

export default NewsletterModal;
