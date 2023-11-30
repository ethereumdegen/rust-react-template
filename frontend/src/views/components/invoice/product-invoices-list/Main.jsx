import {
    Lucide, 
  } from "@/base-components";
 
  import axios from "axios";

 
  import { useState, useEffect } from 'react';


 import { useNavigate } from 'react-router-dom';
 
import ProductRow from "@/views/components/product/product-row/Main.jsx";
 

import { getBackendServerUrl } from '@/lib/app-helper'
import {observe} from 'mobx'
import { observer } from "mobx-react";
 
import Modal from '@/views/components/modal/Main.jsx'

import AutoForm from '@/views/components/autoform/Main.jsx'

import {createProject} from '@/lib/project-lib'

import InvoicesList from "@/views/components/invoice/invoices-list/Main.jsx"

function ProductInvoicesList({web3Store, productId, onInvoicesChanged}) {

  const [invoices, invoicesSet] = useState(null) 

  let navigate = useNavigate(); 


  const fetchInvoices = async () => {
    console.log('start fetch invoices')
    const backendApiUri = `${getBackendServerUrl()}/v1/invoices_by_product`
    let response = await axios.get(backendApiUri,{
      params:{
        productId: productId,
        publicAddress: web3Store.account,
        authToken: web3Store.authToken 
      }
    }) 

    if(!response || !response.data ) return undefined 

    console.log({response})
    let invoices = response.data.data

    return invoices 
  }
  


   const loadInvoices = async (newFilter) => {
    console.log('loading invoices') 
       
        try{ 
          const invoices = await fetchInvoices()
          console.log({invoices})

          invoicesSet(invoices)
        }catch(e){
          console.error(e)
        }
   }
 

   observe(web3Store, 'account', function() {
    console.log('acct:', web3Store.account); 
  });

  //load   on authorized 
  observe(web3Store, 'authorized', function() {
    console.log('acct:', web3Store.account);
   
    loadInvoices()
  });
   

 //load  on mount 
 useEffect(()=>{ 
  loadInvoices()
}, []) // <-- empty dependency array

 
   


  return (
     <div>
     


        {invoices &&  

        
            <div className=" ">
            <InvoicesList 
              invoices = {invoices}  
             />  

            </div>
         }



         
     

  </div>
  );

}


export default observer(ProductInvoicesList);
