
import axios from "axios";

import {
  Lucide, 
} from "@/base-components";

import { useState, useEffect } from 'react';
 
import { useOutletContext, useParams, useNavigate } from 'react-router-dom';

import { observer } from "mobx-react";
import {observe} from 'mobx'

import { Tab } from "@/views/base-components/Headless";
import SimpleButton from '@/views/components/button/SimpleButton'
 
import TinyBadge from "@/views/components/tiny-badge/Main"

import { getBackendServerUrl } from '@/lib/app-helper'

import ProductInvoicesList from "@/views/components/invoice/product-invoices-list/Main"
import SignInRequiredWarning from "@/views/components/sign-in-required-warning/Main"


function Main(  ) {
  
  let navigate = useNavigate(); 
     
    const [web3Store] = useOutletContext(); // <-- access context value

    console.log('web3Store' , web3Store)
 
 
    const [product, productSet] = useState(null) 


    const { productId } = useParams();
  

  const fetchProduct = async () => {
   
    const backendApiUri = `${getBackendServerUrl()}/v1/product`
    let response = await axios.get(backendApiUri,{
      params:{
        productId,
        publicAddress: web3Store.account,
        authToken: web3Store.authToken 
      }
    }) 

    if(!response || !response.data ) return undefined 

    console.log({response})
    let product = response.data.data

    return product 
  }
  

   const loadProduct = async (newFilter) => {
    console.log('loading product')
       
        try{ 
          const product = await fetchProduct()
          console.log({product})

          productSet(product)
        }catch(e){
          console.error(e)
        }
   }

   observe(web3Store, 'account', function() {
    console.log('acct:', web3Store.account); 
  });
  
  observe(web3Store, 'authorized', function() {
    console.log('acct:', web3Store.account);
    loadProduct()
  });
   

 //load api keys on mount 
 useEffect(()=>{
  loadProduct()
}, []) // <-- empty dependency array

 



  return (
    <>
      <div className="intro-y flex flex-col sm:flex-row items-center mt-2">
       
      </div>
      <div className="intro-y box pt-4 px-5 pb-4 mt-2 flex flex-col items-center">
      
     
 

        <div className="pt-4 px-2 pb-16 w-full">
      
      
        {/* BEGIN:   Title */}
        {product && 
        <div className=" mt-2 mb-5 ">
          <div className="text-xl   my-2 ">
          {product.name}
          </div>
          <TinyBadge
            customClass="my-2 bg-black text-white"
          >
           product
          </TinyBadge>
         
          <a href="" className="  block text-primary text-base">
             
          </a>
        </div>
        }
        {/* END: Tx Title */}
        {/* BEGIN: Tx Content */}

        <div className="w-full">

 


        <SignInRequiredWarning
        authorized={web3Store.authorized}            
      >
          Sign in to view your product
      </SignInRequiredWarning> 

      
        
        {web3Store.authorized && product &&

          <div> 
            <div className="inline-block">

            <SimpleButton
              customClass="hover:bg-slate-300 flex flex-row inline-block"
              clicked={() => navigate(`/dashboard/product/${product._id}/invoice/new`)}  
              >  
              <Lucide icon="PlusCircle" className="w-4 h-4 mr-1" />
                
                Add a new Invoice
              </SimpleButton> 

            </div>

        <div className="flex flex-col">

         
            <div className="px-4 py-16 text-lg font-bold">
              Invoices for this Product
            </div>
             

            <div>

              <ProductInvoicesList
                productId={product._id}
                web3Store={web3Store}
              />

 

            </div>
         
         </div>
         
          </div>
         }
          
          

          
        </div>
            
    
        {/* END: Tx Content */}
      </div>
      </div>

    </>
  );
}

export default observer(Main);


 