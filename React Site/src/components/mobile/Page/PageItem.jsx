import './page.css';

function PageItem({ page, creator, getPage }) {
  const dateCreated = new Date(page.date_created);
  return (
    <>
      <div
        className="mobile-page-item"
        onClick={() => getPage(page.id, creator)}
      >
        <p className="mobile-page-item-name">{page.title}</p>
        <p className="mobile-page-item-created_by">
          Created By <span>{creator}</span>
        </p>
        <p className="mobile-page-item-date-created">
          Created on the {dateCreated.getDay()}/{dateCreated.getMonth()}/
          {dateCreated.getFullYear()}
        </p>
      </div>
    </>
  );
}

export default PageItem;
