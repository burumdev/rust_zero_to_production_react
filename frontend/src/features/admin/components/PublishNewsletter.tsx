import type { AdminPublishNewsletterRequest } from '@admin/admin_types';
import { useAdminPublishNewsletter } from '@admin/hooks/use_admin';
import Button from '@components/Button';
import TextInput from '@components/TextInput';
import { useEffect, useState } from 'react';
import type { ValidationRuleSet } from 'src/hooks/useFormValidator';
import useFormValidator from 'src/hooks/useFormValidator';

type FormData = AdminPublishNewsletterRequest;

const INITIAL_FORM_DATA: FormData = {
  title: '',
  content: '',
};

const FORM_VALIDATION_RULES: ValidationRuleSet<FormData> = {
  title: [
    {
      ruleName: 'empty',
      errorStr: 'Title can not be empty',
    },
    {
      ruleName: 'max',
      max: 100,
      errorStr: 'Can not be more than 100 chars',
    },
  ],
  content: [
    {
      ruleName: 'empty',
      errorStr: 'Content can not be empty',
    },
    {
      ruleName: 'max',
      max: 3000,
      errorStr: 'Can not be more than 3000 chars',
    },
  ],
};

const PublishNewsletter: React.FC = () => {
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
  } = useAdminPublishNewsletter();

  const handleSubmit = () => {
    if (isFormValid()) {
      sendForm(formData as AdminPublishNewsletterRequest);
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
    <div className="p-4 step-box bg-darkgray">
      <h1>Publish Newsletter</h1>
      <div className="text-darkgray">
        <div className="mt-6 h-forminput">
          <TextInput
            placeholder="Title*"
            isFocused
            onChange={(e: any) => setFormField('title', e.target.value)}
            value={formData.title}
            // Normally form validator supports multiple error messages
            // But for UX and UI purposes we pluck the first error
            errorStr={formErrors['title']?.[0].errorStr}
          />
        </div>
        <div className="mt-6 h-forminput">
          <TextInput
            placeholder="Content*"
            onChange={(e: any) => setFormField('content', e.target.value)}
            value={formData.content}
            errorStr={formErrors['content']?.[0].errorStr}
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
            text="PUBLISH"
            backgroundClass="bg-highlight"
            dropShadowColor="highlight-darker"
            isDisabled={isFormSubmitButtonDisabled || isPending}
            isLoading={isPending}
          />
        </div>
      </div>
    </div>
  );
};

export default PublishNewsletter;
