import { useState, useEffect } from 'react';
import './page-settings.css';

function PageDetailsEditor({
  sessionUser,
  token,
  APIAdress,
  selectedPage,
  setSelectedPage,
  getPage,
  pageCss,
  getPageCss,
  pageCsses,
}) {
  var [editing, setEditing] = useState(false);

  var [title, setTitle] = useState(selectedPage.title);
  var [published, setPublished] = useState(selectedPage.published);
  var [cssSheet, setCssSheet] = useState(selectedPage.selected_css_id);

  var [error, setError] = useState(null);

  useEffect(() => {
    setTitle(selectedPage.title);
    setPublished(selectedPage.published);
    setCssSheet(selectedPage.selected_css_id);
  }, [selectedPage, editing]);

  var tl = title;
  var pub = published;
  var pcss = cssSheet;

  async function updatePageDetails() {
    if (!validateUpdateInput()) {
      return;
    }

    const settings = {
      method: 'PUT',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        session_user_id: sessionUser.id,
        page_id: selectedPage.id,
        published: Boolean(published),
        title: title,
        selected_css_id: Number(cssSheet),
      }),
    };
    try {
      const fetchUpdatePage = await fetch(APIAdress + `pages/page`, settings);

      if (!fetchUpdatePage.ok && fetchUpdatePage.status !== 400) {
        throw new Error('Cannot Connect to Server');
      }

      const response = await fetchUpdatePage.json();
      if (response.success == true) {
        setError('');
        getPage(selectedPage.id);
        setEditing(false);
        getPageCss();
      } else {
        setError(response.msg);
      }
    } catch (e) {
      console.log(e.message);
      setError('Cannot Connect to Server');
    }
  }

  async function deletePage() {
    if (
      confirm(
        'Are you sure you want to delete this page? This is not Salesforce, you cannot revert this with Undelete. If you do not know what this means you are likely a moron and should cancel and get an adult.'
      )
    ) {
    } else {
      return;
    }

    const settings = {
      method: 'DELETE',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        session_user_id: sessionUser.id,
        page_id: selectedPage.id,
      }),
    };

    try {
      const fetchDeletePage = await fetch(APIAdress + `pages/page`, settings);

      if (!fetchDeletePage.ok && fetchDeletePage.status !== 400) {
        throw new Error('Cannot Connect to Server');
      }

      const response = await fetchDeletePage.json();
      if (response.success == true) {
        setError('');
        setSelectedPage(null);
      } else {
        setError(response.msg);
      }
    } catch (e) {
      console.log(e.message);
      setError('Cannot Connect to Server');
    }
  }

  function validateUpdateInput() {
    if (title == '') {
      setError('Title Field Cannot be Blank.');
      return false;
    } else {
      return true;
    }
  }

  function discard() {
    setTitle(selectedPage.title);
    setPublished(selectedPage.published);
    setCssSheet(selectedPage.selected_css_id);
    setEditing(false);
    setError('');
  }

  if (
    pageCsses.length > 0 &&
    selectedPage != null &&
    selectedPage != undefined
  ) {
    if (editing) {
      return (
        <>
          <div className="settings-section">
            <br></br>
            <h3 className="section-title">Edit Page Details</h3>
            <br></br>
            <div className="edit-window">
              <p className="settings-error">{error}</p>
              <div className="edit-page-details">
                <p>Title</p>
                {title === '' ? (
                  <>
                    <input
                      className="edit-title"
                      style={{ background: '#FFE5E5' }}
                      type="text"
                      placeholder="Uses of Potatoes"
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
                      className="edit-title"
                      type="text"
                      placeholder="Uses of Potatoes"
                      name="title"
                      required
                      value={tl}
                      onChange={(e) => {
                        setTitle(e.target.value);
                      }}
                    ></input>
                  </>
                )}
                <p>Published</p>
                <input
                  className="edit-published"
                  style={{ background: '#FFE5E5' }}
                  type="checkbox"
                  name="published"
                  required
                  checked={pub}
                  onChange={(e) => {
                    setPublished(e.target.checked);
                  }}
                ></input>

                <p>Css Sheet</p>
                <select
                  className="edit-select-sheet"
                  style={{ background: '#FCFFFF' }}
                  value={pcss}
                  onChange={(e) => setCssSheet(e.target.value)}
                >
                  {pageCsses.map((pageCs) => (
                    <option key={pageCs.id} value={pageCs.id}>
                      {pageCs.sheet_name}
                    </option>
                  ))}
                </select>
              </div>
              <br></br>
              <div className="page-details-save-or-discard">
                <button
                  className="settings-save"
                  onClick={() => updatePageDetails()}
                >
                  Save
                </button>
                <button className="settings-discard" onClick={() => discard()}>
                  Discard
                </button>
              </div>
              <br></br>
            </div>
          </div>
        </>
      );
    } else {
      return (
        <>
          <div className="settings-section">
            <br></br>
            <h3 className="section-title">Page Details</h3>
            <br></br>
            <div className="page-details">
              <p>
                <span>Title:</span> {selectedPage.title}
              </p>
              <p>
                <span>
                  {selectedPage.published === true ? (
                    <>Published</>
                  ) : (
                    <>Unpublished</>
                  )}
                </span>
              </p>
              <p>
                <span>CSS Sheet:</span> {pageCss.sheet_name}
              </p>
            </div>
            <div className="page-details-edit-or-delete">
              <button
                className="edit-page-details"
                onClick={() => setEditing(true)}
              >
                <svg
                  width="24px"
                  height="24px"
                  viewBox="0 0 24 24"
                  xmlns="http://www.w3.org/2000/svg"
                >
                  <path
                    fill-rule="evenodd"
                    clip-rule="evenodd"
                    d="M8.56078 20.2501L20.5608 8.25011L15.7501 3.43945L3.75012 15.4395V20.2501H8.56078ZM15.7501 5.56077L18.4395 8.25011L16.5001 10.1895L13.8108 7.50013L15.7501 5.56077ZM12.7501 8.56079L15.4395 11.2501L7.93946 18.7501H5.25012L5.25012 16.0608L12.7501 8.56079Z"
                  />
                </svg>
              </button>
              <button className="settings-del" onClick={() => deletePage()}>
                <svg
                  width="800px"
                  height="800px"
                  viewBox="0 0 24 24"
                  xmlns="http://www.w3.org/2000/svg"
                >
                  <path
                    d="M10 11V17"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                  />
                  <path
                    d="M14 11V17"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                  />
                  <path
                    d="M4 7H20"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                  />
                  <path
                    d="M6 7H12H18V18C18 19.6569 16.6569 21 15 21H9C7.34315 21 6 19.6569 6 18V7Z"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                  />
                  <path
                    d="M9 5C9 3.89543 9.89543 3 11 3H13C14.1046 3 15 3.89543 15 5V7H9V5Z"
                    stroke-width="2"
                    stroke-linecap="round"
                    stroke-linejoin="round"
                  />
                </svg>
              </button>
            </div>
          </div>
        </>
      );
    }
  }
}

export default PageDetailsEditor;
