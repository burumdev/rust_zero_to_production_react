import type { TabOptionSettings } from '@components/TabOption';
import Tabs from '@components/Tabs';
import { useState } from 'react';

const tabOptions: TabOptionSettings[] = [
  {
    text: 'Chat Moderation',
  },
  {
    text: 'Account',
  },
];

const AdminPanel = () => {
  const [activeTabIndex, setActiveTabIndex] = useState(0);
  return (
    <div className="w-full flex flex-col">
      <h1 className="text-center">Admin Panel</h1>
      <div className="w-[400px] self-center mt-4">
        <Tabs
          options={tabOptions}
          activeIndex={activeTabIndex}
          onChange={(index) => setActiveTabIndex(index)}
        />
      </div>
      {activeTabIndex === 0 && (
        <div className="w-full h-full mt-4 px-4">
          <div className="p-4 step-box bg-darkgray">Chat Rooms</div>
        </div>
      )}
      {activeTabIndex === 1 && (
        <div className="w-full h-full mt-4 px-4">
          <div className="p-4 step-box bg-darkgray">Account</div>
        </div>
      )}
    </div>
  );
};

export default AdminPanel;
