import type { TabOptionSettings } from '@components/TabOption';
import Tabs from '@components/Tabs';
import { Outlet } from 'react-router';

const tabOptions: TabOptionSettings[] = [
  {
    path: '/adminpanel/chat',
    text: 'Chat',
  },
  {
    path: '/adminpanel/newsletters',
    text: 'Newsletters',
  },
  {
    path: '/adminpanel/account',
    text: 'Account',
  },
];

const AdminPanelLayout: React.FC = () => {
  return (
    <div className="w-full flex flex-col">
      <h1 className="text-center">Admin Panel</h1>
      <div className="w-[400px] self-center mt-4">
        <Tabs options={tabOptions} />
      </div>

      <div
        className={`w-full h-full mt-4 px-4${isSplitHalf && ' grid grid-cols-2 gap-4'}`}
      >
        <Outlet />
      </div>
    </div>
  );
};

export default AdminPanelLayout;
