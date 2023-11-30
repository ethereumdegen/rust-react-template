
import axios from 'axios'


import { getBackendServerUrl} from '../../lib/app-helper'



export const broadcastTransaction = async (web3Store, txId) => {

    console.log('broadcast tx  ', txId)

    const backendApiUri = `${getBackendServerUrl()}/transaction`
    let response = await axios.post(backendApiUri,{
      txId,
      publicAddress: web3Store.account,
      authToken: web3Store.authToken,
      action:'broadcast'

    }) 

    console.log({response})

  }

export const denyTransaction = async (web3Store, txId) => {

    console.log('deny tx  ', txId)

    const backendApiUri = `${getBackendServerUrl()}/transaction`
    let response = await axios.post(backendApiUri,{
      txId,
      publicAddress: web3Store.account,
      authToken: web3Store.authToken,
      action:'deny'

    }) 


    console.log({response})
  }



export function txIsConfirmable(tx,transactionCount){

  return (tx && tx.networkStatus == 'pending' && tx.nonce == transactionCount)

}