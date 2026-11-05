import { NavLink } from 'react-router';

const Header = () => {
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
        <NavLink
          to="/login"
          className={({ isActive }) => (isActive ? 'text-highlight' : '')}
        >
          LOGIN
        </NavLink>
      </nav>
    </header>
  );
};

export default Header;
