import { Link } from 'react-router-dom';
import './main.css';
import { useState } from 'react';

function NavBar({ token, sessionUser, show, setShow }) {
  if (token && sessionUser && show) {
    return (
      <>
        <div className="mobile-navbar">
          <div className="mobile-nav-item-section">
            <h2 className="mobile-nav">Routes</h2>
            <Link className="mobile-nav" to="/">
              Home
            </Link>
            <Link className="mobile-nav" to="/pages">
              Pages
            </Link>
          </div>
          <br></br>
          <br></br>
          {sessionUser.user_type_id <= 2 ? (
            <>
              <div className="mobile-nav-item-section">
                <h2 className="mobile-nav" style={{ fontSize: '5px' }}>
                  Administrator Routes
                </h2>
                <Link
                  className="mobile-nav"
                  to="/manage-users"
                  style={{ fontSize: '5px' }}
                >
                  Manage Users
                </Link>
              </div>
            </>
          ) : (
            <></>
          )}

          <br></br>
          <br></br>
          <button className="mobile-close" onClick={() => setShow(false)}>
            &lt;
          </button>
        </div>
      </>
    );
  } else if (token && sessionUser && !show) {
    return (
      <>
        <div>
          <button className="mobile-open" onClick={() => setShow(true)}>
            &gt;
          </button>
        </div>
      </>
    );
  }
}

export default NavBar;
