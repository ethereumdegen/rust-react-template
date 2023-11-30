import axios from "axios"
import { getBackendServerUrl } from "../lib/app-helper"

import {BigNumber} from 'ethers'
import backendRoutes from "../config/routes.json"
import qs from 'qs'; 

let backendRouteLookup = {}



function loadBackendRoutes () {

    for (let route of backendRoutes){

        backendRouteLookup[route.method] = route

    }

}

loadBackendRoutes()


export function getBackendRouteFromMethodName(methodName:string){
 
    let routeExt = backendRouteLookup[methodName].uri

    return routeExt
}

export function getRouteTypeFromMethodName(methodName:string){
   
    return backendRouteLookup[methodName].type
}

export function getEtherscanAccountLink(
    {publicAddress,chainId}
    :{publicAddress:string,chainId:string}
    ) : string {

        

        return `${getEtherscanBaseUrl({chainId})}/account/${publicAddress}`
    }

export function getEtherscanTransactionLink(
    {transactionHash,chainId}
    :{transactionHash:string,chainId:string}
    ) : string {


        return `${getEtherscanBaseUrl({chainId})}/tx/${transactionHash}`
    }

export function getEtherscanBaseUrl({chainId}:{chainId:string}) : string {

    chainId = BigNumber.from(chainId).toString()

    switch(chainId){
        case "11155111": return "https://sepolia.etherscan.io"
        case "5": return "https://goerli.etherscan.io"
        default: return "https://etherscan.io"
    }
}

export function getNetworkName({chainId}:{chainId:number}) : string {
 

    switch(parseInt(chainId.toString())){
        case 11155111: return "Sepolia"
        case 5: return "Goerli"
        case 1: return "Mainnet"
        case 0: return "Any Network"

        default: return "Unknown Network"
    }

}


export function getDateFormatted({date,seconds}:{date?:number,seconds?:number}) : string {
    
    if(date){
        return new Date(date).toLocaleString()
    }
    if(seconds){
        return new Date(seconds*1000).toLocaleString()
    }
    
    return "Unknown Date"
}




export async function backendCallForRoute(
    {
        methodName,
        endpointParams,
   
        headers,
         

    } :
    {
        methodName: string,
        endpointParams:any // params for get or body for post
     
        headers?:any 
      
    }
){ 
    
    let restMethod = getRouteTypeFromMethodName(methodName)
    let endpointURI = getBackendRouteFromMethodName(methodName) 
 

    return await backendCall({
        restMethod,
        endpointURI,
    
        endpointParams,
        headers 
 
    })
}


export async function backendCall(
    {
        restMethod,
        endpointURI,
    
        endpointParams,
        headers,
     
    }:{
        restMethod:string // get or post 
        endpointURI:string 
   
        endpointParams:any // params for get or body for post 
        headers?:any
      
    }
     
    ) : Promise<any> {

    
            console.log(restMethod, endpointParams)

            const backendApiUri = `${getBackendServerUrl()}${endpointURI}`
     
           

           try {
             let response = await axios({
                method: restMethod,
                url: backendApiUri,
                params: restMethod.toLowerCase() === 'get' ? endpointParams : undefined,
                paramsSerializer: {
                    indexes: null // by default: false
                  }, 
                data: restMethod.toLowerCase() === 'post' ? endpointParams : undefined,
                headers
            })
              console.log("axios response ", response)

            return response 
            } catch (response:any) {
                console.log("axios error", response);

                return response?.response
            }
           

      


    }