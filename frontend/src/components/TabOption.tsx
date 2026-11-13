export type TabOptionSettings = {
  text?: string;
  icon_url?: string;
  icon_alt?: string;
};

export interface TabOptionProps extends TabOptionSettings {
  key: string;
  index: number;
  onClick: (index: number) => void;
  isActive: boolean;
}

const TabOption: React.FC<TabOptionProps> = ({
  text,
  icon_url,
  icon_alt,
  key,
  index,
  onClick,
  isActive,
}) => {
  return (
    <div
      className={`tab-option cursor-pointer${isActive ? ' text-highlight' : ''}`}
      key={key}
      onClick={() => onClick(index)}
    >
      <div
        className={`p-[8px] step-box border-2${isActive ? ' bg-darkgray' : ''}`}
      >
        {text && <span>{text}</span>}
        {icon_url && (
          <img src={icon_url} alt={icon_alt || `Tab option ${index + 1}`} />
        )}
      </div>
    </div>
  );
};

export default TabOption;
