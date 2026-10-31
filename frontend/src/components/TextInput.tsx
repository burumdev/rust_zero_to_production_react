import { useEffect, useRef } from 'react';
import './TextInput.css';

interface TextInputProps {
  placeholder?: string;
  isFocused?: boolean;
  onChange: (e: any) => void;
  value: string;
  errorStr?: string;
}

const TextInput: React.FC<TextInputProps> = ({
  placeholder,
  isFocused,
  onChange,
  value,
  errorStr = '',
}) => {
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (isFocused && inputRef.current) {
      inputRef.current.focus();
    }
  }, [isFocused]);

  return (
    <div className="relative w-full h-full">
      <div className="z-1 container-textinput absolute step-box bg-yellow w-full h-full"></div>
      <input
        ref={inputRef}
        className="relative z-2 w-full h-full px-3 placeholder-yellow-100"
        placeholder={placeholder || undefined}
        onChange={(e) => onChange(e)}
        value={value}
      />
      {errorStr !== '' && (
        <div className="filter-none text-sm text-error">{errorStr}</div>
      )}
    </div>
  );
};

export default TextInput;
