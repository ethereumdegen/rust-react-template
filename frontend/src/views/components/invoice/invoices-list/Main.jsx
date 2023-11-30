import {
    Lucide, 
  } from "@/base-components";
 

  import { useState } from 'react';


 import { useNavigate } from 'react-router-dom';
 import SimpleButton from '@/views/components/button/SimpleButton'
 
import InvoiceRow from "@/views/components/invoice/invoice-row/Main.jsx";

import { observer } from "mobx-react";
 
 
 

function InvoicesList({web3Store, invoices, onInvoicesChanged}) {

  let navigate = useNavigate();
 
 

  return (
     <div> 

        {invoices && invoices.map((item,index)=>{ 

        return (
           
           
            <InvoiceRow
            className="my-8"
            key={index}
            web3Store ={web3Store}
            invoiceData = {item} 
                    
            />  
            
        )

        })} 
     

  </div>
  );

}


export default observer(InvoicesList);
