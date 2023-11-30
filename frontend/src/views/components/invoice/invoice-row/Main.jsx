import {
    Lucide, 
  } from "@/base-components";
 

import { useNavigate } from 'react-router-dom';
 
import { observer } from "mobx-react";

function InvoiceRow({web3Store, invoiceData}) {

  let navigate = useNavigate();

  
  console.log({invoiceData})


 

  return (
    <div className="border-slate-200 border-2 rounded p-4 my-4 w-full">
     
    <div className="flex flex-col">
    <div className="flex flex-row w-full">  
         

        <div className="flex flex-grow flex-col ">
         {invoiceData && <div className="flex flex-row my-2">
            <div className='font-bold  '> {invoiceData.invoiceUUID } </div>
          
          </div> }

        
          
       </div>


       <div className="flex flex-row"> 
        <div> 
          {invoiceData && <div 
          className={`mx-4 p-2 capitalize font-bold bg-slate-200 border-2 border-gray-400 cursor-pointer`}
          onClick={()=>{navigate(`/dashboard/invoice/${invoiceData.invoiceUUID}`)}}
          > 
            View
          </div>}
         </div>
        
         </div> 
      
    </div>

    <div className="flex flex-row w-full flex-grow"> 

      <div className="flex flex-grow"></div>
     
    </div>

    </div>
    
    </div>
  )

}

  


export default observer(InvoiceRow);


