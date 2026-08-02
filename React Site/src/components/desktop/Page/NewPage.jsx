import { useState } from 'react';

function NewPage({
  APIAdress,
  token,
  sessionUser,
  switchComponent,
  showMessage,
  getPage,
}) {
  const [title, setTitle] = useState('');

  var [error, setError] = useState(null);

  var tl = title;

  async function newPage() {
    if (!validateInput()) {
      return;
    }

    const settings = {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        created_by_id: Number(sessionUser.id),
        title: String(title),
      }),
    };

    console.log(settings);

    try {
      const fetchNewPage = await fetch(APIAdress + `pages/page/post`, settings);

      if (!fetchNewPage.ok && fetchNewPage.status !== 400) {
        throw new Error('Cannot Connect to Server');
      }

      const response = await fetchNewPage.json();
      console.log(response);
      if (response.success == true) {
        showMessage('Created page Successfully!');
        getPage(response.data);
      } else {
        setError(response.msg);
      }
    } catch (e) {
      console.log(e.message);
      setError('Cannot Connect to Server');
    }
  }

  function validateInput() {
    console.log(name);
    if (title == '') {
      setError('Title cannot be blank.');
      return false;
    } else {
      return true;
    }
  }

  async function discard() {
    setTitle('');
    switchComponent(0);
  }

  return (
    <>
      <div className="new-page">
        <h1 className="new-page">Create New Page</h1>
        <br></br>
        <div>
          <h2 className="new-page">Page Title</h2>
          <p>Enter a page title below to create your own page.</p>
          <p className="page-error">{error}</p>
          {title === '' ? (
            <>
              <input
                className="new-page"
                style={{ background: '#FFE5E5' }}
                type="text"
                placeholder="The Wonderous World of Limpets"
                name="title"
                required
                value={tl}
                onChange={(e) => {
                  setTitle(e.target.value);
                }}
              ></input>
            </>
          ) : (
            <>
              <input
                className="new-page"
                type="text"
                placeholder="The Wonderous World of Limpets"
                name="title"
                required
                value={tl}
                onChange={(e) => {
                  setTitle(e.target.value);
                }}
              ></input>
            </>
          )}
        </div>
        <br></br>
        <br></br>
        <br></br>
        <div className="new-page-save-or-discard">
          <button
            className="new-page-discard "
            onClick={() => {
              discard();
            }}
          >
            Discard
          </button>
          <button
            className="new-page-save"
            onClick={() => {
              newPage();
            }}
          >
            Save
          </button>
        </div>
      </div>
    </>
  );
}

export default NewPage;
