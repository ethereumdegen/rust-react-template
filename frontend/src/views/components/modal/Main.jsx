import { useState } from 'react';


import { observer } from "mobx-react";
function Modal({isOpen, closeModal, title,  children}) {
 
 

  return (
    <>
     

      {isOpen && (
         <div className="absolute z-50 h-full w-full">
         <div className="flex justify-center items-center h-screen">
        <div className="fixed z-20 inset-0 overflow-y-auto bg-black bg-opacity-50" onClick={closeModal}>
          <div className="flex items-center justify-center min-h-screen" >
            <div className="bg-white w-11/12 sm:w-8/12 lg:w-6/12 xl:w-4/12 rounded-lg shadow-lg overflow-hidden" onClick={(e) => e.stopPropagation()}>
              <div className="bg-black text-white px-4 py-3 flex flex-row">

                <h3 className="font-bold text-lg flex-grow "> {title} </h3>
                <button onClick={closeModal} className="px-2  focus:outline-none">
                  <span className="text-lg">&times;</span>
                </button>

              </div>
              <div className="p-4">
                
                   {children}
             
              </div>
            </div>
          </div>
        </div>
        </div>
        </div>
      )}
    </>
  );
}


export default observer(Modal);