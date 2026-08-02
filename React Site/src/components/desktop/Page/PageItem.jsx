import './page.css';

function PageItem({ page, creator, getPage }) {
  const dateCreated = new Date(page.date_created);
  return (
    <>
      <div className="page-item" onClick={() => getPage(page.id, creator)}>
        <p className="page-item-name">{page.title}</p>
        <p className="page-item-created_by">
          Created By <span>{creator}</span>
        </p>
        <p className="page-item-date-created">
          Created on the {dateCreated.getDay()}/{dateCreated.getMonth()}/
          {dateCreated.getFullYear()}
        </p>
      </div>
    </>
  );
}

export default PageItem;
