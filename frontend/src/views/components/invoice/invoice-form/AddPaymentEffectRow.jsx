import { useState } from 'react';

import { observer } from "mobx-react";

import {
    Lucide, 
  } from "@/base-components";
 
 
 
/*
This is used for adding new effects to an invoice 

*/

function AddPaymentEffectRow({  currentRowData, productOptions , onUpdatedProductReferenceId, onUpdatedTargetPublicAddress, onRemoveRow   }) {
  const [formData, setFormData] = useState({});

  {/*
   add value ..
  */}

  return (
    <div className="flex flex-row p-2 my-2 "> 

      <select
      
      name="productReferenceId"
      placeholder="Product Reference Id"
      value={currentRowData.productReferenceId}
      required={true}
      className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
      onChange={(event)=>{onUpdatedProductReferenceId(event.target.value)}}
      >

          {productOptions.map((option) => (
                <option key={option.value} value={option.value}>
                  {option.label}
                </option>
          ))}

      </select>
     

      <input
        type="text"
        name="targetPublicAddress"
        placeholder="Target Public Address"
        value={currentRowData.targetPublicAddress}
        required={true}
        className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
        onChange={(event)=>{onUpdatedTargetPublicAddress(event.target.value)}}
      />

      <div className="p-2 mx-4 rounded bg-slate-700">



      <Lucide 
      icon="X"
      className="w-6 h-6 mr-1 cursor-pointer rounded bg-transparent font-bold text-white"

      color="white"
      onClick={ onRemoveRow }
      />
               

      </div>
      

        
    </div>
    
     
  );
}



export default observer(AddPaymentEffectRow);