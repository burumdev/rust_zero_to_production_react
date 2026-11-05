import { useEffect, useRef } from 'react';
import './TextInput.css';

interface TextInputProps {
  placeholder?: string;
  isFocused?: boolean;
  onChange: (e: any) => void;
  value: string;
  errorStr?: string;
  type?: 'text' | 'password';
}

const TextInput: React.FC<TextInputProps> = ({
  placeholder,
  isFocused,
  onChange,
  value,
  errorStr = '',
  type = 'text',
}) => {
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (isFocused && inputRef.current) {
      inputRef.current.focus();
    }
  }, [isFocused]);

  return (
    <div className="w-full h-full">
      <div className="textinput w-full h-full">
        <input
          ref={inputRef}
          className="step-box px-3 placeholder-yellow-100 bg-yellow w-full h-full"
          placeholder={placeholder || undefined}
          onChange={(e) => onChange(e)}
          value={value}
          type={type}
        />
      </div>
      {errorStr !== '' && (
        <div className="mt-1 text-sm text-error">{errorStr}</div>
      )}
    </div>
  );
};

export default TextInput;
