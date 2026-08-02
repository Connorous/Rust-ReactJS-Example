import { useNavigate } from 'react-router-dom';
import { useEffect } from 'react';

function Logout({ setToken, setSessionUser }) {
  const navigate = useNavigate();
  useEffect(() => {
    setToken(null);
    setSessionUser(null);
    navigate('/login');
  }, []);
}

export default Logout;
