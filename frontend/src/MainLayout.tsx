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
    <>
      <Header />
      <div className="flex justify-center">
        <Outlet />
      </div>
    </>
  );
};

export default MainLayout;
