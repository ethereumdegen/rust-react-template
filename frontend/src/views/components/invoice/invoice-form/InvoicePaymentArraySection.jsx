import { useState } from 'react';

import { observer } from "mobx-react";

import {
    Lucide, 
  } from "@/base-components";
 
  import SimpleButton from '@/views/components/button/SimpleButton'
 
 import InvoicePaymentRow from './InvoicePaymentRow';

function InvoicePaymentElementSection({  onUpdated   }) {
  const [paymentRows, setPaymentRows] = useState( [] );

  
  const addPaymentRow = () => {


    let newPaymentRows = [...paymentRows]
    newPaymentRows.push({
      payTo: '0x...',
      amountDue: '0'
    })

    setPaymentRows(newPaymentRows)
   onUpdated('paymentRowsData',[...newPaymentRows])

  }

  const removePaymentRow= (index) => {

    let newPaymentRows = [...paymentRows]
    
    newPaymentRows.splice(index)

    setPaymentRows(newPaymentRows)
     onUpdated('paymentRowsData',[...newPaymentRows])

  }


  const updatePaymentRowInput = (index,fieldName,value) => {

    const paymentRowsData = [...paymentRows]

    paymentRowsData[index] = Object.assign({},paymentRowsData[index]);

    if(fieldName == 'payTo'){
      paymentRowsData[index].payTo = value
    }else if(fieldName == 'amountDue'){
      paymentRowsData[index].amountDue = value
    }else {
      throw new Error('Unknown field type:',fieldName)
    }
 
    //to send back down to children for rendering 
    setPaymentRows([...paymentRowsData])

    //to send to callback up to parent 
    onUpdated('paymentRowsData',[...paymentRowsData])


  }

  return (
    <div className="p-4 my-4 container box "> 

      <div className="font-bold text-lg">
                   
      </div>   

      <div>
      {paymentRows && Array.isArray(paymentRows) &&  paymentRows.map((paymentRow, index) => (
           <InvoicePaymentRow
            key={index}
              currentRowData={paymentRow}
              onUpdatedPayToAddress={(updatedAddress) => updatePaymentRowInput(index,'payTo',updatedAddress)}
              onUpdatedPayToAmount={(updatedAmount) => updatePaymentRowInput(index,'amountDue',updatedAmount)}
              onRemoveRow={()=>{removePaymentRow(index)}}
              /> 
      ))}
      </div>  

      <div>

      <div className="inline-block">


        <SimpleButton
        customClass={"hover:bg-gray-700 hover:text-white flex flex-row text-lg  "}
        clicked={() => {
          addPaymentRow()
        }}
        
        >
         <Lucide icon="PlusCircle" className="w-8 h-8 mr-2" />

          Add new payment row
        </SimpleButton>
      </div>
      </div>
   

    </div>
    
     
  );
}



export default observer(InvoicePaymentElementSection);