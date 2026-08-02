function UserTable({ users, userTypes, selectUser, switchComponent }) {
  return (
    <>
      <div className="manage-users-table">
        <button className="manage-users-add" onClick={() => switchComponent(2)}>
          +
        </button>
        <br></br>
        <br></br>
        <br></br>
        <table className="manage-users-table">
          <thead>
            <tr>
              <th>Username</th>
              <th>Name</th>
              <th>Email</th>
              <th>Date Created</th>
              <th>User Type</th>
            </tr>
          </thead>
          <tbody>
            {/* The map() method creates a row for each data object */}

            {users.length === 0 || userTypes.length == 0 ? (
              <>
                <tr>
                  <td>Loading table</td>
                  <td></td>
                  <td></td>
                  <td></td>
                  <td></td>
                </tr>
              </>
            ) : (
              <></>
            )}

            {users.map((user) => (
              <tr
                key={user.id}
                onClick={() => {
                  selectUser(user);
                }}
              >
                <td>{user.username}</td>
                <td>{user.name}</td>
                <td>{user.email}</td>
                <td>{user.date_created}</td>
                <td>
                  {user.user_type_id === userTypes[0].id ? (
                    <>{userTypes[0].type}</>
                  ) : (
                    <></>
                  )}
                  {user.user_type_id === userTypes[1].id ? (
                    <>{userTypes[1].type}</>
                  ) : (
                    <></>
                  )}
                  {user.user_type_id === userTypes[2].id ? (
                    <>{userTypes[2].type}</>
                  ) : (
                    <></>
                  )}
                  {user.user_type_id === userTypes[3].id ? (
                    <>{userTypes[3].type}</>
                  ) : (
                    <></>
                  )}
                  {user.user_type_id === userTypes[4].id ? (
                    <>{userTypes[4].type}</>
                  ) : (
                    <></>
                  )}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
        <br></br>
      </div>
    </>
  );
}

export default UserTable;
