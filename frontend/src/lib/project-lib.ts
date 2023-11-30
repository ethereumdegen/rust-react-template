import { Provider } from "@ethersproject/abstract-provider"
import axios from "axios"
import { ethers } from "ethers"

  
import { getBackendServerUrl } from "./app-helper"

export async function createProject({
    name, publicAddress, authToken, onFinished
}:{
    name: string
    publicAddress:string,
    authToken:string,

    onFinished: ( ) => any
}){
    console.log('creating project', name )

    const backendApiUri = `${getBackendServerUrl()}/v1/project`
    let response = await axios.post(backendApiUri,{
        name,
        publicAddress,//: web3Store.account,
        authToken//: web3Store.authToken 
      
    }) 

    if(!response || !response.data ) return undefined 

    console.log({response})

    onFinished() 

}

/*
const createProject = async () => {
      
    const backendApiUri = `${getBackendServerUrl()}/v1/project`
    let response = await axios.post(backendApiUri,{
        
        publicAddress: web3Store.account,
        authToken: web3Store.authToken 
      
    }) 

    if(!response || !response.data ) return undefined 

    console.log({response})

    loadProjects()
 
  }



export async function getMintCount(networkName:string, provider: Provider){

    let localConfig:any = contractsConfig[networkName]
    
    let jpegsContract = new ethers.Contract(localConfig['jpegsNFT'].address, JpegsNftAbi, provider)

    let count =  await jpegsContract.mintedTokenCount()

    return count.toString()
}


export async function getTokenUriExtension(tokenId: string, networkName:string, provider: Provider){

    let localConfig:any = contractsConfig[networkName]
    
    let jpegsContract = new ethers.Contract(localConfig['jpegsNFT'].address, JpegsNftAbi, provider)

    return await jpegsContract.uriExtensions(tokenId)
}



export async function getOwnerOf(tokenId: string, networkName:string, provider: Provider){

    let localConfig:any = contractsConfig[networkName]
    
    let jpegsContract = new ethers.Contract(localConfig['jpegsNFT'].address, JpegsNftAbi, provider)

    return await jpegsContract.ownerOf(tokenId)
}*/