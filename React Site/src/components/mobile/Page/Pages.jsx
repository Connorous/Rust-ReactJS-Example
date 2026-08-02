import { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import './page.css';
import PageList from './PageList';
import NewPage from './NewPage';

function Pages({
  APIAdress,
  token,
  sessionUser,
  setSelectedPage,
  setSelectedPageCreator,
}) {
  var [viewListorPageorNew, setviewListorPageorNew] = useState(0);

  var [responseMsg, setResponseMsg] = useState('');
  var [showResponseMsg, setShowResponseMsg] = useState(false);

  var [error, setError] = useState('');

  const navigate = useNavigate();
  useEffect(() => {
    if (!token) {
      navigate('/login');
    }
  }, []);

  useEffect(() => {
    if (sessionUser.user_type_id > 4) {
      console.log('not allowed');
      navigate('/');
    }
  }, []);

  async function showMessage(msg) {
    setResponseMsg(msg);
    setShowResponseMsg(true);
    setTimeout(hideMessage, 5000);
  }

  async function hideMessage() {
    setResponseMsg('');
    setShowResponseMsg(false);
  }

  async function getPage(page_id, creator) {
    const settings = {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        session_user_id: Number(sessionUser.id),
        page_id: Number(page_id),
      }),
    };

    try {
      const fetchPage = await fetch(APIAdress + `pages/page/get`, settings);

      if (!fetchPage.ok && fetchPage.status !== 400) {
        throw new Error('Cannot Connect to Server');
      }

      const response = await fetchPage.json();
      if (response.success == true) {
        const page = response.data;
        setSelectedPage(page);
        setSelectedPageCreator(creator);
        navigate('/page');
        setError('');
      } else {
        setError(response.msg);
      }
    } catch (e) {
      console.log(e.message);
      setError('Cannot Connect to Server');
    }
  }

  async function selectPage(page) {
    setSelectedUser(page);
    switchComponent(1);
  }

  async function switchComponent(section) {
    setviewTableorUserorAddUser(section);
  }

  async function showMessage(msg) {
    setResponseMsg(msg);
    setShowResponseMsg(true);
    setTimeout(hideMessage, 5000);
  }

  async function hideMessage() {
    setResponseMsg('');
    setShowResponseMsg(false);
  }

  async function switchComponent(section) {
    setviewListorPageorNew(section);
  }

  if (sessionUser.user_type_id <= 4) {
    return (
      <>
        <div className="mobile-pages">
          <p className="mobile-page-error">{error}</p>
          {showResponseMsg ? (
            <>
              <p>{responseMsg}</p>
            </>
          ) : (
            <></>
          )}

          {viewListorPageorNew === 0 ? (
            <>
              <div>
                <PageList
                  APIAdress={APIAdress}
                  token={token}
                  sessionUser={sessionUser}
                  switchComponent={switchComponent}
                  getPage={getPage}
                ></PageList>
              </div>
            </>
          ) : (
            <></>
          )}
          {viewListorPageorNew === 1 ? (
            <>
              <NewPage
                APIAdress={APIAdress}
                token={token}
                sessionUser={sessionUser}
                switchComponent={switchComponent}
                showMessage={showMessage}
                getPage={getPage}
              ></NewPage>
            </>
          ) : (
            <></>
          )}
        </div>
      </>
    );
  }
}

export default Pages;
