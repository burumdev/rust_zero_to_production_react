import { useAdminLogout, useLoginState } from '@admin/hooks/use_admin';
import { useEffect } from 'react';
import { NavLink, useNavigate } from 'react-router';

const Header = () => {
  const isLoggedIn = useLoginState();
  const { mutate: postLogout, isSuccess: isLogoutSuccess } = useAdminLogout();
  const navigate = useNavigate();

  const onLogoutClick = () => {
    postLogout();
  };

  useEffect(() => {
    if (isLogoutSuccess) {
      sessionStorage.clear();
      navigate('/');
    }
  }, [isLogoutSuccess]);

  return (
    <header className="h-20 flex">
      <nav className="flex w-full items-center px-4">
        <NavLink to="/" className="m-0 m-auto">
          <img
            src="/images/heading.svg"
            alt="Zero to Production Rust + React"
            className="p-5"
          />
        </NavLink>
        {isLoggedIn ? (
          <button className="cursor-pointer" onClick={() => onLogoutClick()}>
            LOGOUT
          </button>
        ) : (
          <NavLink
            to="/login"
            className={({ isActive }) => (isActive ? 'text-highlight' : '')}
          >
            LOGIN
          </NavLink>
        )}
      </nav>
    </header>
  );
};

export default Header;
