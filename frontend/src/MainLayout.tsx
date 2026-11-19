import Header from '@components/Header';
import { apiFetch } from '@utils/api_client';
import { useEffect } from 'react';
import { Outlet } from 'react-router';

const MainLayout = () => {
  // Fetch the CSRF cookie manually in cross-origin setup
  useEffect(() => {
    if (import.meta.env.MODE === 'development') {
      apiFetch('csrf_cookie');
    }
  }, []);

  return (
    <div className="w-full h-full flex justify-center">
      <div className="w-[1536px]">
        <Header />
        <div className="flex justify-center">
          <Outlet />
        </div>
      </div>
    </div>
  );
};

export default MainLayout;
