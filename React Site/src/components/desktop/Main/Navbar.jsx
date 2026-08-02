import { Link } from 'react-router-dom';
import './main.css';
import { useState } from 'react';

function NavBar({ token, sessionUser, show, setShow }) {
  if (token && sessionUser && show) {
    return (
      <>
        <div className="navbar">
          <div className="nav-item-section">
            <h2 className="nav">Routes</h2>
            <Link className="nav" to="/">
              Home
            </Link>
            <Link className="nav" to="/pages">
              Pages
            </Link>
          </div>
          <br></br>
          <br></br>
          {sessionUser.user_type_id <= 2 ? (
            <>
              <div className="nav-item-section">
                <h2 className="nav">Administrator Routes</h2>
                <Link className="nav" to="/manage-users">
                  Manage Users
                </Link>
              </div>
            </>
          ) : (
            <></>
          )}

          <br></br>
          <br></br>
          <button className="close" onClick={() => setShow(false)}>
            &lt;
          </button>
        </div>
      </>
    );
  } else if (token && sessionUser && !show) {
    return (
      <>
        <div className="nav-bar-hidden">
          <button className="open" onClick={() => setShow(true)}>
            &gt;
          </button>
        </div>
      </>
    );
  }
}

export default NavBar;
