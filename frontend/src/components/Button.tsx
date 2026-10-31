interface ButtonProps {
  onClick: () => void;
  backgroundClass?: string;
  dropShadowColor?: string;
  text?: string;
  isDisabled?: boolean;
  isLoading?: boolean;
}

const Button: React.FC<ButtonProps> = ({
  onClick,
  text,
  backgroundClass = 'bg-yellow',
  dropShadowColor = 'yellow',
  isDisabled = false,
  isLoading = false,
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
        disabled={isDisabled || isLoading}
      >
        <span className={isLoading ? 'blinking' : undefined}>{text}</span>
      </button>
    </div>
  );
};

export default Button;
