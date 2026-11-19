import { Link } from 'react-router';

export type TabOptionSettings = {
  path: string;
  text?: string;
  icon_url?: string;
  icon_alt?: string;
};

export interface TabOptionProps extends TabOptionSettings {
  isActive: boolean;
}

const TabOption: React.FC<TabOptionProps> = ({
  path,
  text,
  icon_url,
  icon_alt,
  isActive,
}) => {
  return (
    <Link
      to={path}
      className={`tab-option cursor-pointer${isActive ? ' text-highlight' : ''}`}
    >
      <div
        className={`p-[8px] step-box border-2${isActive ? ' bg-darkgray' : ''}`}
      >
        {text && <span>{text}</span>}
        {icon_url && (
          <img src={icon_url} alt={icon_alt || `Tab option ${path}`} />
        )}
      </div>
    </Link>
  );
};

export default TabOption;
