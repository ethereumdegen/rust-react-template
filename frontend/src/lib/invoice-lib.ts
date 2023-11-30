 
import axios from "axios"
 

import { Web3Provider } from '@ethersproject/providers';
//export async function userPayInvoice( from:string, invoiceData: PayspecInvoice, provider: Web3Provider, netName?: string ) : Promise<{success:boolean, error?:any, data?: any}> {


import { getBackendServerUrl } from "./app-helper"


import {
 
    PayspecPaymentElement,
    
    PayspecPaymentEffect,

    userPayInvoice,
  
    PayspecInvoice,
     
    generatePayspecInvoiceSimple
} from 'payspec-js'
import { AssertionResult } from "../interfaces/types";

 

/*


  payspecContractAddress: '0x...',   -- AUTOGENERATED 
  description: 'Invoice for services',
  nonce: '0x...',   -- AUTOGENERATED   
  token: '0x...',
  chainId: '0', 
  payToArrayStringified: '["0x..."]',  -- GENERATED FROM PAYMENTS ARRAY
  amountsDueArrayStringified: '["1000000000000000000"]',   -- GENERATED FROM PAYMENTS ARRAY
  expiresAt: 1234567890   -- AUTOGENERATED 


*/

enum PayspecEffectType {

    GRANT_PRODUCT_ACCESS 
    
}


interface PayspecPaymentEffect{

    type: PayspecEffectType, 

    invoiceUUID: string, //triggers when this invoice is paid 



    referenceId: string, //a product id for example 

    targetPublicAddress: string, 


}

/*

 
*/

export async function addInvoice({
    chainId, 
    description, 
    tokenAddress, 
    paymentsArray,

    paymentEffects,

    ownerAddress,
    authToken,
    
   /// onFinished
}:{
    chainId: number,
    description: string,
    tokenAddress:string, 
    paymentsArray:PayspecPaymentElement[],

    paymentEffects: PayspecPaymentEffect[], 

    ownerAddress: string, 
    authToken:string, 

   // onFinished: ( result:AssertionResult<string|undefined> ) => any
}) : Promise<AssertionResult<string|undefined> >{


    let invoice:PayspecInvoice

    try{
    invoice = generatePayspecInvoiceSimple(
        {
            chainId,
            description,
            tokenAddress,
            paymentsArray
        }
    ) 
    }catch(err:any){
        return {success:false, error: err.toString()}
    }

    const backendApiUri = `${getBackendServerUrl()}/v1/invoice`
    let response = await axios.post(backendApiUri,{
        invoice,

        paymentEffects,

        publicAddress: ownerAddress,//: web3Store.account,
        authToken//: web3Store.authToken 
      
    }) 

    console.log({response})
 

    if(!response || !response.data) return {success:false,error:"Invalid response from API"} 

    if(!response.data.success){
        return {success:false, error:response.data.error}
        
    }
 
    return {success:true, data:invoice.invoiceUUID} 

}


export async function payInvoiceUsingProvider({
    from,
    invoice,
    provider,
    networkName
}:{
    from:string,
    invoice: PayspecInvoice,
    provider: Web3Provider,
    networkName: string 
}){

    let paid = await userPayInvoice(
        from,
        invoice,
        provider,
        networkName
    )

    return paid 

}