import { is_valid_email } from '@utils/validators';
import { useEffect, useState } from 'react';

export type FormDataObject = Record<string, string>;
export type ValidationRuleName = 'empty' | 'email' | 'min' | 'same' | 'notSame';

type ValidationRuleBase<FDT> = {
  ruleName: ValidationRuleName;
  errorStr: string;
  samenessKey?: keyof FDT & string;
  min?: number;
};

export type ValidationRule<FDT> =
  | (ValidationRuleBase<FDT> & {
      ruleName: 'empty';
    })
  | (ValidationRuleBase<FDT> & {
      ruleName: 'email';
    })
  | (ValidationRuleBase<FDT> & {
      ruleName: 'min';
      min: number;
    })
  | (ValidationRuleBase<FDT> & {
      ruleName: 'same';
      samenessKey: keyof FDT & string;
    })
  | (ValidationRuleBase<FDT> & {
      ruleName: 'notSame';
      samenessKey: keyof FDT & string;
    });

export type ValidationRuleSet<FDT> = Record<keyof FDT, ValidationRule<FDT>[]>;
type ValidationErrors<FDT> = Partial<Record<keyof FDT, ValidationRule<FDT>[]>>;

const validate = <FDT>(
  value: string,
  rule: ValidationRule<FDT>,
  otherValue: string | undefined = undefined,
): boolean => {
  switch (rule.ruleName) {
    case 'email':
      return is_valid_email(value);
    case 'empty':
      return value.length > 0;
    case 'min':
      return value.length > 0 && value.length >= rule.min;
    case 'same':
      return value === otherValue;
    case 'notSame':
      return value !== otherValue;
  }
};

const useFormValidator = <FDT extends FormDataObject>(
  initialFormData: FDT,
  validationRules: ValidationRuleSet<FDT>,
  validateInitital: boolean = false,
) => {
  const [formData, setFormData] = useState<FDT>(initialFormData);
  const [formErrors, setFormErrors] = useState<ValidationErrors<FDT>>({});

  // Validate the initial form data
  useEffect(() => {
    if (validateInitital) {
      for (const [key, value] of Object.entries(formData)) {
        validateFormField(key, value);
      }
    }
  }, []);

  const validateFormField = (fdKey: keyof FDT & string, fdValue: string) => {
    if (validationRules[fdKey]) {
      const rules = validationRules[fdKey];

      setFormErrors((prev) => {
        const next = { ...prev };

        rules.forEach((rule) => {
          const foundIndex =
            next[fdKey] &&
            next[fdKey].findIndex((ru) => ru.ruleName === rule.ruleName);

          const isValid = ['same', 'notSame'].includes(rule.ruleName)
            ? validate(fdValue, rule, formData[rule.samenessKey as string])
            : validate(fdValue, rule);

          if (isValid) {
            if (next[fdKey] && foundIndex >= 0) {
              next[fdKey].splice(foundIndex, 1);
            }
            if (next[fdKey] && next[fdKey].length === 0) {
              delete next[fdKey];
            }
          } else {
            if (next[fdKey] && foundIndex === -1) {
              next[fdKey] = [...next[fdKey], rule];
            } else {
              next[fdKey] = [rule];
            }
          }
        });

        return next;
      });
    }
  };

  const resetForm = () => {
    setFormData(initialFormData);
  };

  const isFormValid = () => {
    return (
      Object.keys(formErrors).length === 0 &&
      Object.values(formData).every((item) => item !== '')
    );
  };

  const setFormField = (field: keyof FDT, value: any) => {
    setFormData({
      ...formData,
      [field]: value,
    });
    validateFormField(field as string, value);
  };

  return {
    formData,
    formErrors,
    setFormField,
    resetForm,
    isFormValid,
  };
};

export default useFormValidator;
