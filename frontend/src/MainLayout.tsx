import Header from '@components/Header';
import { Outlet } from 'react-router';

const MainLayout = () => {
  return (
    <>
      <Header />
      <div className="flex justify-center">
        <Outlet />
      </div>
    </>
  );
};

export default MainLayout;
