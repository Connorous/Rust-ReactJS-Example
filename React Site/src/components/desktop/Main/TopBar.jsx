import { useNavigate } from 'react-router-dom';
import './main.css';

function TopBar({ token, sessionUser }) {
  const navigate = useNavigate();

  function logout() {
    navigate('/logout');
  }

  if (token && sessionUser) {
    return (
      <>
        <div className="top-bar">
          <div className="top-bar-details">
            <h2>Page Creator 2.1</h2>
            {sessionUser.user_type_id <= 4 ? (
              <>
                <h4>Welcome {sessionUser.username}!</h4>
              </>
            ) : (
              <>
                <h4>Fuck Off {sessionUser.username}!</h4>
              </>
            )}
          </div>
          <button className="logout" onClick={() => logout()}>
            Logout
          </button>
        </div>
      </>
    );
  }
}

export default TopBar;
