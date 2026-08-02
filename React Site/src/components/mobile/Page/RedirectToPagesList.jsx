import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';

function RedirectToPagesList({ selectedPage }) {
  const navigate = useNavigate();
  useEffect(() => {
    if (selectedPage == null || selectedPage == undefined) {
      navigate('/pages');
    }
  }, []);
}

export default RedirectToPagesList;
