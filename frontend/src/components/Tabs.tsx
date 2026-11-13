import { v6 as uuidv6 } from 'uuid';

import type { TabOptionSettings } from './TabOption';
import TabOption from './TabOption';

interface TabsProps {
  options: TabOptionSettings[];
  activeIndex: number;
  onChange: (index: number) => void;
}

const Tabs: React.FC<TabsProps> = ({ options, activeIndex, onChange }) => {
  return (
    <div className="w-full h-full flex justify-between">
      {options.map((opt, index) => {
        return (
          <TabOption
            {...opt}
            key={uuidv6()}
            index={index}
            isActive={activeIndex === index}
            onClick={(index) => onChange(index)}
          />
        );
      })}
    </div>
  );
};

export default Tabs;
