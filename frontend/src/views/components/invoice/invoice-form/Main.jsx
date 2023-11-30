import { useState } from 'react';

import { observer } from "mobx-react";

import {
    Lucide, 
  } from "@/base-components";
 
 import AutoForm from '@/views/components/autoform/Main'
 
 import InvoicePaymentArraySection from './InvoicePaymentArraySection'

 import PaymentEffectsArraySection from './PaymentEffectsArraySection'

  const architecture = {
    fields:[

      {
        name: 'name',
        type: 'text',
        label: 'Name',
        placeholder: 'Name',
        required: true 
      },

      {
        name: 'chainId',
        type: 'select',
        label: 'Network',
        placeholder: 'Network',
        required: true ,
        options:[
          {label: 'Ethereum Mainnet', value: 1},
          {label: 'Ethereum Sepolia', value: 11155111},
          {label: 'Ethereum Goerli', value: 5},
        ]
      },

      {
        name: 'tokenName',
        type: 'select',
        label: 'Token',
        placeholder: 'Token',
        required: true ,
        options:[
          {label: 'Ether', value: 'ether'},
          {label: '0xBTC', value: '0xbtc'},
         
        ]
      },

      {
        name: 'paymentElementArray',
        type: InvoicePaymentArraySection, 
        label: 'Payment Elements',
       

      },


      {
        name: 'paymentEffectsArray',
        type: PaymentEffectsArraySection, 
        label: 'Payment Effects',
       

      }

    ]


  }
 


function InvoiceForm({ web3Store, onSubmit }) {
  const [formData, setFormData] = useState({});

  

  return (
    <AutoForm 
    
    architecture={architecture}
    
    
    web3Store={web3Store}

    
    onSubmit={ async( formData ) =>{

      await onSubmit(formData)
      console.log('on submit invoice')

     

      /*
      let added = await addInvoice( {
        chainId: formData.network,
        description: formData.name,
        tokenAddress ,
        paymentsArray ,


      })*/

    }}
    
    />
     
  );
}



export default observer(InvoiceForm);