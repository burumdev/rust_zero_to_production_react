import { v6 as uuidv6 } from 'uuid';
import { useLocation } from 'react-router';

import type { TabOptionSettings } from './TabOption';
import TabOption from './TabOption';

interface TabsProps {
  options: TabOptionSettings[];
}

const Tabs: React.FC<TabsProps> = ({ options }) => {
  const location = useLocation();

  return (
    <div className="w-full h-full flex justify-between">
      {options.map((opt) => {
        return (
          <TabOption
            {...opt}
            key={uuidv6()}
            isActive={location.pathname === opt.path}
          />
        );
      })}
    </div>
  );
};

export default Tabs;
