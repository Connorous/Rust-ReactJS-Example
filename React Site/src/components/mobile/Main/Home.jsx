import { useState, useEffect } from 'react';
import uuid from 'react-uuid';

function Home({ token }) {
  useEffect(() => {
    if (!token) {
      navigate('/login');
    }
  }, []);

  return (
    <>
      <div className="mobile-home">
        <h1>Page Creator</h1>
        <h3>
          Welcome to Page Creator Version 2.1! Not that that means anything!{' '}
          <br></br>
          <span>
            {'('}There haven't been any previous versions, And Yes, I had to use
            two "That"s, couldn't help myself{')'}
          </span>
        </h3>
        <p>
          Use the Navigation on the side to Navigate to See Pages you have
          permission to View, or Create your own.
          <br></br>
          <br></br>
          On your own Page you may:
          <br></br>
          Create Certain types of Page HTML Elements,
          <br></br>
          Define CSS Styling for the Page Elements and Switch CSS Sheets,
          <br></br>
          and even Manage Page Permissions, Assigning View/Edit Permissions to
          who you want to be able to see or edit your Page.
          <br></br>
          <br></br>
          Once Done, Update your Page to Published for others with permission to
          be able to View it.
        </p>
      </div>
    </>
  );
}

export default Home;
