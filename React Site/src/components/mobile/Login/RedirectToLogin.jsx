import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';

function RedirectToLogin({ token, sessionUser }) {
  const navigate = useNavigate();
  useEffect(() => {
    if (token == null && sessionUser == null) {
      console.log('asd');
      navigate('/login');
    }
  }, []);
}

export default RedirectToLogin;
