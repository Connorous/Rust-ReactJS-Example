import { useNavigate } from 'react-router-dom';
import { useState, useEffect, useRef } from 'react';
import uuid from 'react-uuid';

function Home({ token }) {
  const navigate = useNavigate();
  useEffect(() => {
    if (!token) {
      navigate('/login');
    }
  }, []);

  const childDiv = useRef(null);

  useEffect(() => {
    if (childDiv.current) {
      const parentElement = childDiv.current.parentElement;
      if (parentElement) {
        parentElement.className = 'none';
      }
    }
  }, []);

  const [secondsLeft, setSecondsLeft] = useState(10);

  useEffect(() => {
    let interval = null;

    if (secondsLeft > 0) {
      interval = setInterval(() => {
        setSecondsLeft((prevTime) => prevTime - 1);
      }, 1000);
    } else if (secondsLeft === 0) {
      navigate('/logout');
    }

    return () => clearInterval(interval);
  }, [secondsLeft]);

  return (
    <>
      <div ref={childDiv} className="blocked">
        <h1>You've Been a Bad Human</h1>
        <p>
          Haha, you've been naughty. You're now banned from this page, please
          logout. If you don't you will be forced to logout in {secondsLeft}.
        </p>
      </div>
    </>
  );
}

export default Home;
