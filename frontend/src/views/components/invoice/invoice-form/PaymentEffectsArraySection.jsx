import { useState, useEffect } from 'react';

import { observer  } from "mobx-react";
import {observe} from 'mobx'

import axios from 'axios'

import {
    Lucide, 
  } from "@/base-components";
 
  import SimpleButton from '@/views/components/button/SimpleButton'
 
 import AddPaymentEffectRow from './AddPaymentEffectRow';

 import { getBackendServerUrl } from '@/lib/app-helper'



function PaymentEffectsArraySection({ web3Store, onUpdated   }) {
  const [effectRows, setEffectRows] = useState( [] );


  //load this from api ? 
  const [productOptions, setProductOptions] = useState( [] );


  


  const fetchProducts = async () => {
   
    const backendApiUri = `${getBackendServerUrl()}/v1/products_by_owner`
    let response = await axios.get(backendApiUri,{
      params:{ 
        publicAddress: web3Store.account,
        authToken: web3Store.authToken 
      }
    }) 

    if(!response || !response.data ) return undefined 

    console.log({response})
    let products = response.data.data

    return products
  }
  

   const loadProducts = async (newFilter) => {
    console.log('loading products')
       
        try{ 
          const products = await fetchProducts()
        

          let productOptions = products.map((product)=>{
            return {
              value: product._id,
              label: product.name
            }})
            console.log({productOptions})

          setProductOptions(productOptions)
        }catch(e){
          console.error(e)
        }
   }


 
  
  const addEffectRow = () => {


    const initialProductReferenceId = productOptions[0].value ? productOptions[0].value : undefined 


    let newEffectRows = [...effectRows]
    newEffectRows.push({
        productReferenceId: initialProductReferenceId,
        targetPublicAddress: '0x...'
    })

    setEffectRows(newEffectRows)
    onUpdated('effectRowsData',[...newEffectRows])

  }

  const removeEffectRow= (index) => {

    let newEffectRows = [...effectRows]
    
    newEffectRows.splice(index)

    setEffectRows(newEffectRows)
    onUpdated('effectRowsData',[...newEffectRows])

  }


  const updateEffectRowInput = (index,fieldName,value) => {

    const effectRowsData = [...effectRows]

    effectRowsData[index] = Object.assign({},effectRowsData[index]);

    if(fieldName == 'productReferenceId'){
        effectRowsData[index].productReferenceId = value
    }else if(fieldName == 'targetPublicAddress'){
        effectRowsData[index].targetPublicAddress = value
    }else {
      throw new Error('Unknown field type:',fieldName)
    }
 
    //to send back down to children for rendering 
    setEffectRows([...effectRowsData])

    //to send to callback up to parent 
    onUpdated('effectRowsData',[...effectRowsData])


  }


  observe(web3Store, 'authorized', function() {
    console.log('acct:', web3Store.account);
    loadProducts()
  });
   

  //load on mount 
  useEffect(()=>{
    loadProducts()
  }, []) // <-- empty dependency array

 



  return (
    <div className="p-4 my-4 container box "> 

      <div className="font-bold text-lg">
                   
      </div>   

      <div>
      {effectRows && Array.isArray(effectRows) &&  effectRows.map((effectRow, index) => (
           <AddPaymentEffectRow
              key={index}
              currentRowData={effectRow}
              productOptions={productOptions}
              onUpdatedProductReferenceId={(updatedProductReferenceId) => updateEffectRowInput(index,'productReferenceId',updatedProductReferenceId)}
              onUpdatedTargetPublicAddress={(updatedTargetPublicAddress) => updateEffectRowInput(index,'targetPublicAddress',updatedTargetPublicAddress)}
              onRemoveRow={()=>{removeEffectRow(index)}}
              /> 
      ))}
      </div>  

      <div>

      <div className="inline-block">


        <SimpleButton
        customClass={"hover:bg-gray-700 hover:text-white flex flex-row text-lg  "}
        clicked={() => {
          addEffectRow()
        }}
        
        >
         <Lucide icon="PlusCircle" className="w-8 h-8 mr-2" />

          Add new payment effect
        </SimpleButton>
      </div>
      </div>
   

    </div>
    
     
  );
}



export default observer(PaymentEffectsArraySection);