interface ButtonProps {
  onClick: () => void;
  backgroundClass?: string;
  dropShadowColor?: string;
  text?: string;
  isDisabled?: boolean;
  isLoading?: boolean;
  children?: React.ReactNode;
}

const Button: React.FC<ButtonProps> = ({
  onClick,
  text,
  backgroundClass = 'bg-yellow',
  dropShadowColor = 'yellow',
  isDisabled = false,
  isLoading = false,
  children = null,
}) => {
  return (
    <div
      className="h-full"
      style={{
        filter: `drop-shadow(4px 4px 0 var(--color-${dropShadowColor}))`,
      }}
    >
      <button
        className={`step-box flex justify-center items-center h-full px-3 cursor-pointer ${backgroundClass}`}
        onClick={() => onClick()}
        disabled={isDisabled}
      >
        {text && (
          <span className={isLoading ? 'blinking' : undefined}>{text}</span>
        )}
        {children}
      </button>
    </div>
  );
};

export default Button;
