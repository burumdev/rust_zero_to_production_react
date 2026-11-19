import type { TabOptionSettings } from '@components/TabOption';
import Tabs from '@components/Tabs';
import AccountTab from './AccountTab';
import NewslettersTab from './NewslettersTab';
import { Route, Routes } from 'react-router';

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

const AdminPanel = () => {
  return (
    <div className="w-full flex flex-col">
      <h1 className="text-center">Admin Panel</h1>
      <div className="w-[400px] self-center mt-4">
        <Tabs options={tabOptions} />
      </div>
      <Routes>
        <Route
          path="chat"
          element={
            <div className="w-full h-full mt-4 px-4">
              <div className="p-4 step-box bg-darkgray">Chat Rooms</div>
            </div>
          }
        />
        <Route path="newsletters" element={<NewslettersTab />} />
        <Route path="account" element={<AccountTab />} />
      </Routes>
    </div>
  );
};

export default AdminPanel;
