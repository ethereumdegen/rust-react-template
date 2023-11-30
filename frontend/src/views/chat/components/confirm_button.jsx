import React, { useState } from 'react';

const Main = ({ label, action, performButtonAction }) => {
  const [actionSuccessful, setActionSuccessful] = useState(false);

  return (
    <>
      {!actionSuccessful && (
        <div
          className="bg-blue-500 hover:bg-blue-400 text-white px-4 py-2 rounded cursor-pointer mx-auto"
          onClick={async () => {
            const successful = await performButtonAction(action ?? "");
            if (successful) {
              setActionSuccessful(true);
            }
          }}
        >
          {label ?? "Confirm"}
        </div>
      )}

    {actionSuccessful && (

        <div
          className="bg-white-500 text-black px-4 py-2 font-bold mx-auto"
          
        >
          Command successful.
        </div>

    )}
    </>
  );
};

export default Main;
