import { useState, useEffect } from 'react';
import PageItem from './PageItem';
import './page.css';

function PageList({ APIAdress, token, sessionUser, switchComponent, getPage }) {
  var [pages, setPages] = useState([]);
  var [pageCreators, setPageCreators] = useState([]);
  var [selectedCreator, setSelectedCreator] = useState('');
  var [currentPageSection, setCurrentPageSection] = useState(1);

  var [error, setError] = useState('');

  let sc = selectedCreator;

  useEffect(() => {
    getAllPages();
    getAllPageCreators();
  }, []);

  async function getAllPages() {
    const settings = {
      method: 'GET',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
    };

    try {
      const fetchPages = await fetch(
        APIAdress + `pages/list/` + sessionUser.id,
        settings
      );

      if (!fetchPages.ok && fetchPages.status !== 400) {
        throw new Error('Cannot Connect to Server');
      }

      const response = await fetchPages.json();
      if (response.success == true) {
        const pages = response.data;
        setPages(pages);
        setError('');
      } else {
        setError(response.msg);
      }
    } catch (e) {
      console.log(e.message);
      setError('Cannot Connect to Server');
    }
  }

  async function getCreatorPages() {
    if (selectedCreator == 'None') {
      getAllPages();
      return;
    }

    const settings = {
      method: 'POST',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({
        session_user_id: Number(sessionUser.id),
        pages_user_id: Number(selectedCreator),
      }),
    };

    try {
      const fetchPages = await fetch(
        APIAdress + `pages/list-pages-usermade`,
        settings
      );

      if (!fetchPages.ok && fetchPages.status !== 400) {
        throw new Error('Cannot Connect to Server');
      }

      const response = await fetchPages.json();
      if (response.success == true) {
        const pages = response.data;
        setPages(pages);
        setError('');
      }
    } catch (e) {
      console.log(e.message);
      setError('Cannot Connect to Server');
    }
  }

  async function getAllPageCreators() {
    const settings = {
      method: 'GET',
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
    };

    try {
      const fetchPageCreators = await fetch(
        APIAdress + `pages/list-creators/` + sessionUser.id,
        settings
      );

      if (!fetchPageCreators.ok && fetchPageCreators.status !== 400) {
        throw new Error('Cannot Connect to Server');
      }

      const response = await fetchPageCreators.json();

      if (response.success == true) {
        const creators = response.data;
        setPageCreators(creators);
        setError('');
      } else {
        setError(response.msg);
      }
    } catch (e) {
      console.log(e.message);
      setError('Cannot Connect to Server');
    }
  }

  function getCreatorUsername(created_by_id) {
    if (pageCreators.length == 1) {
      return pageCreators[0].username;
    } else if (pageCreators.length > 0) {
      for (let i = 0; i < pageCreators.length; i++) {
        let current = pageCreators[i];
        if (current.id == created_by_id) {
          return current.username;
          break;
        }
      }
    } else {
      return '';
    }
  }

  var pagesDisplayedLimit = 10;

  var lastIndex = currentPageSection * pagesDisplayedLimit;
  var firstIndex = lastIndex - pagesDisplayedLimit;

  var pagesToDisplay = pages.slice(firstIndex, lastIndex);

  var totalSections = Math.ceil(pages.length / pagesDisplayedLimit);
  if (totalSections > 20) {
    totalSections = 20;
  }

  // Handle page changes
  function handlePageChange(pageNumber) {
    if (!(pageNumber < 0)) {
      setCurrentPageSection(pageNumber);
    }
  }

  if (pages.length > 0 && pageCreators.length > 0) {
    return (
      <>
        <div className="pages">
          <h1 className="pages-title">Pages</h1>
          <p>
            Pages you have permission to see will be listed here. Filter by
            Creator to reduce number of results.
          </p>
          <p className="error">{error}</p>
          <br></br>
          <div className="page-list-actions">
            {sessionUser.user_type_id <= 3 ? (
              <>
                <button className="new-page" onClick={() => switchComponent(1)}>
                  Create Page +
                </button>
              </>
            ) : (
              <></>
            )}
            <div className="pages-filter">
              <select
                className="pages-creator-filter"
                value={sc}
                onChange={(e) => setSelectedCreator(e.target.value)}
              >
                <option value={null}>None</option>

                {pageCreators.map((creator) => (
                  <option key={creator.id} value={creator.id}>
                    {creator.username}
                  </option>
                ))}
              </select>
              <button
                className="pages-filter"
                onClick={() => getCreatorPages()}
              >
                Filter
              </button>
            </div>
          </div>

          <div className="pages-list">
            {pages.length > 1 ? (
              <>
                <p className="page-results">
                  Results: {pages.length} Pages Found
                </p>
              </>
            ) : (
              <>
                <p className="page-results">
                  Results: {pages.length} Page Found
                </p>
              </>
            )}
            {totalSections > 1 ? (
              <>
                <p className="page-section">
                  {' '}
                  Currently Showing Section {currentPageSection} of Results
                </p>
              </>
            ) : (
              <></>
            )}

            {totalSections > 1 ? (
              <>
                <div className="page-section-select">
                  {Array.from({ length: totalSections }, (_, i) => (
                    <button
                      key={i + 1}
                      onClick={() => handlePageChange(i + 1)}
                      className="page-section-select"
                    >
                      {i + 1}
                    </button>
                  ))}
                </div>
              </>
            ) : (
              <></>
            )}

            <br></br>
            {pagesToDisplay.map((page, index) => (
              <PageItem
                key={page.id}
                page={page}
                creator={getCreatorUsername(page.created_by_id)}
                getPage={getPage}
              ></PageItem>
            ))}
          </div>
        </div>
        <br></br>
        <br></br>
        <br></br>
        <br></br>
      </>
    );
  } else {
    return (
      <>
        <div className="pages">
          <h1 className="pages-title">No Pages Found</h1>
          <p className="error">{error}</p>
          {sessionUser.user_type_id <= 3 ? (
            <>
              <button className="new-page" onClick={() => switchComponent(1)}>
                Create Page +
              </button>
            </>
          ) : (
            <></>
          )}
          <p>
            No pages found, none may exist or you not have permission to view
            any.
          </p>
        </div>
      </>
    );
  }
}

export default PageList;
