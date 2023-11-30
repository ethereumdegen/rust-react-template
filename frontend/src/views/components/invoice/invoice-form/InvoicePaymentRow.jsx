import { useState } from 'react';

import { observer } from "mobx-react";

import {
    Lucide, 
  } from "@/base-components";
 
 
 

function InvoicePaymentRow({  currentRowData, onUpdatedPayToAddress, onUpdatedPayToAmount, onRemoveRow   }) {
  const [formData, setFormData] = useState({});

  {/*
   add value ..
  */}

  return (
    <div className="flex flex-row p-2 my-2 "> 

      <input
      type="text"
      name="payToAddress"
      placeholder="Pay To Address"
      value={currentRowData.payTo}
      required={true}
      className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
      onChange={(event)=>{onUpdatedPayToAddress(event.target.value)}}
      />
     

      <input
        type="text"
        name="payAmount"
        placeholder="Pay Amount"
        value={currentRowData.amountDue}
        required={true}
        className="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
        onChange={(event)=>{onUpdatedPayToAmount(event.target.value)}}
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



export default observer(InvoicePaymentRow);