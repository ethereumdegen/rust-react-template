import { JsonRpcProvider } from '@ethersproject/providers';
import { ethers } from 'ethers';
import React, { useState, useEffect } from 'react';

import  providerConfig from '@/config/provider-config.json'
 
import {getMintCount,getTokenUriExtension,getOwnerOf} from "@/lib/jpeg-lib"
import {getMintPrice} from '@/lib/auction-lib'

import logo0xbtc from '@/assets/images/0xbitcoin-logo-flat2.svg'

import AuctionWallet from '@/views/components/auction-wallet/Main.jsx'

import { ArrowLeftCircle, ArrowRightCircle } from 'lucide-react';

import Axios from 'axios'
import { getAuctionStarted } from '../../lib/auction-lib';

function Auction( {web3Store}  ) {

  let networkName = 'mainnet'
  let provider = new JsonRpcProvider( providerConfig[networkName] )



  const [mintedCount, mintedCountSet] = useState(null) 

  const [tokenOwner, tokenOwnerSet] = useState(null) 

  const [tokenUri, tokenUriSet] = useState(null) 
  const [tokenManifest, tokenManifestSet] = useState(null) 
  const [imageUri, imageUriSet] = useState(null) 
 
  const [mintPrice, mintPriceSet] = useState(null) 
  const [auctionStarted, auctionStartedSet] = useState(null) 

  const [pageNumber, pageNumberSet] = useState(null) 

  let MAX_PAGE_NUMBER = 500 ; 


  let hasInitializedPage = false

  const fetchTokenUri = async(tokenId) => { 

    try{ 
      const ext = await getTokenUriExtension(tokenId, networkName, provider)
     
      let tokenUri = `https://arweave.net/${ext}`
      tokenUriSet(tokenUri)
      
      let manifest = await Axios.get(tokenUri) 
      tokenManifestSet(manifest.data)
       
      let imageUri = manifest.data.image
      imageUriSet(imageUri)
 
    
      return ext 
    }catch(e){
      console.error(e)
     
    }

  }

  const fetchOwnerOf = async(tokenId) => {  

    tokenOwnerSet(undefined)  

    try{ 
      const owner = await getOwnerOf(tokenId, networkName, provider)
      
      tokenOwnerSet(owner) 
      

    }catch(e){
      console.error(e)
    }

  }


 
  const fetchMintedCount = async ( ) => {
    

        try{ 

          const mintCount = await getMintCount(networkName, provider)
          
          mintedCountSet(mintCount)

          if(!hasInitializedPage){
            hasInitializedPage = true 
            setPage(mintCount)
          }
         
        }catch(e){
          console.error(e)
        }
   }
  

   const fetchAuctionStarted = async ( ) => {
 
    try{ 
 
      const auctionStarted = await getAuctionStarted(networkName, provider)
          
      auctionStartedSet(auctionStarted)
    }catch(e){
      console.error(e)
    }
  } 

  const fetchMintPrice = async ( ) => {
 
    try{ 

      const blockNumber = await provider.getBlockNumber();

      const mintPrice = await getMintPrice(blockNumber, networkName, provider)
          
      mintPriceSet(mintPrice)
    }catch(e){
      console.error(e)
    }
  } 

  const setup = async () => {
    await fetchMintedCount()
    await fetchMintPrice()
    await fetchAuctionStarted() 
   
  }

  const getOpenseaAccountPage = (address) => {
    return networkName == 'mainnet' ? `https://opensea.io/${address}` : `https://testnets.opensea.io/${address}`
  }


  // on mount 
  useEffect(  ()=>{
  
    setup()


    let priceInterval = setInterval( fetchMintPrice, 10*1000  )
    let mintCountInterval = setInterval( fetchMintedCount, 20*1000  )

    let pausedCheckInterval = setInterval( fetchAuctionStarted, 20*1000  )
  }, []) // <-- empty dependency array


 const isAvailableToMint = (mintCount,tokenId) => {
  console.log(mintCount,tokenId)
  return parseInt(mintCount) == parseInt(tokenId)
 }  

const setPage = async (newPageNumber) => {
  
  newPageNumber = parseInt(newPageNumber)


  if(newPageNumber < 0 ) newPageNumber = MAX_PAGE_NUMBER;
  if(newPageNumber > MAX_PAGE_NUMBER) newPageNumber = 0;
  

  if(  isNaN(newPageNumber) ) return 

  pageNumberSet(newPageNumber)
 

  //update page 
  let ext = await fetchTokenUri(newPageNumber)
  fetchOwnerOf(newPageNumber)

  if(ext.length == 0 && newPageNumber != 0  ){
    setPage(0)
  }

}

   
  return (
    <section className="relative">

      {networkName == 'goerli' && (web3Store.chainId == '5' || !web3Store.active) &&
     <div className="bg-gray-300 text-black text-center p-2 marquee font-bold ">
       <p> Live on Goerli Testnet </p>  
 
     </div>
      }

      {networkName == 'goerli' && web3Store.active && web3Store.chainId != '5' &&
        <div className="bg-gray-300 text-black text-center p-2   font-bold ">
            <p> Please switch network to Goerli Testnet </p>  
    
        </div>
       }  


    {networkName == 'mainnet' && web3Store.active && web3Store.chainId != '1' &&
        <div className="bg-gray-300 text-black text-center p-2 font-bold ">
            <p> Please switch network to Ethereum Mainnet </p>   
        </div>
       }  
     
      <div className="max-w-6xl mx-auto px-4 sm:px-6">

     

        {/* Hero content */}
        <div className="pt-16 pb-6 md:pt-16 md:pb-8">

          {/* Section header */}
          <div className="t  pb-8 md:pb-8">

         
            <div className="w-full  ">

            <div className="grid lg:grid-cols-2 gap-2">

               
                  <div className="text-center overflow-hidden" style={{ height:'600px'  }}  >
                      <img
                        className="m-auto"
                        src={imageUri} 
                        style={{objectFit:'contain', maxHeight:'600px'}}
                       />
                 </div>

              


             <div >
              <div className="mx-auto px-32">
              {!(isNaN(parseInt(pageNumber))) && 
                  <div className="flex flex-row">

                  <ArrowLeftCircle 
                  color="gray " 
                  size={48}
                  className="cursor-pointer"
                  onClick={()=>{setPage(pageNumber-1)}}
                   />
                  <ArrowRightCircle 
                  color="gray " 
                  size={48}
                  className="cursor-pointer"
                  onClick={()=>{setPage(pageNumber+1)}}
                   />

                  </div> } 
                  </div>

                    <div className="my-8 mx-auto px-32" >
                    {tokenManifest && tokenManifest.attributes && 
                      <div className="flex flex-col">
                      <div className="text-2xl font-bold"> {tokenManifest.name}  </div>
                      <div className="text-lg font-bold"> Token #{tokenManifest.attributes[0].value}  </div>
                      <a className="text-xs text-orange-400" href={tokenUri} > 
                       [view metadata]
                       </a>
                      
                      </div>
                      }

                      {!auctionStarted &&
                        <div className="my-8 box p-4 ">

                          The auction is paused.

                        </div>
                      
                      }

                      { auctionStarted &&  tokenManifest && tokenManifest.attributes &&  isAvailableToMint( mintedCount, tokenManifest.attributes[0].value ) &&  mintPrice &&
                       
                      <div>
                       <div className="my-8 flex flex-col">
                            <div className="underline"> Auction Price (0xbtc)</div>
                            <div className="flex flex-row"> 
                               <div className="px-1 text-lg font-bold"> {  (parseFloat(ethers.utils.formatUnits(mintPrice, 8)) ).toFixed(2) } </div>
                               <div className="px-1" > 
                                <img src={logo0xbtc} width="24" height="24" />
                                </div>
                               </div>

                        </div>


                          <AuctionWallet 
                            web3Store= {web3Store}
                            mintPrice= {mintPrice}
                            networkName = {networkName}
                          
                          />

                        </div>

                        }

                        { tokenOwner && <div 
                        
                        className="my-4"
                        style={{maxWidth:'120px'}}
                        >

                            <div>
                            Collected by: 

                            </div>
                            
                            <a 
                            href={getOpenseaAccountPage(tokenOwner)}
                            className="text-xs text-blue-400 truncate text-ellipsis inline-block"
                            style={{maxWidth:'120px'}}

                            >
                              {tokenOwner}
                            </a>

                          </div>

                        }


                        <div className="my-16 box p-4">

                          <div className="text-xl font-bold"> Fund Allocation </div> 

                        <div className="flex flex-col">
                          <div> 
                            
                          🔥 50% of buyout is burned 
                            
                          </div> 

                          <div> 
                            
                          ⛏️ 50% of buyout is sent to '<a href="https://guild.0xbtc.io" className="text-blue-400">0xBTC Guild</a>' 
                            
                          </div> 

                        </div>
                        </div>

                      </div>
                </div>
                
               
             </div>

              </div>
              <div className="max-w-3xl mx-auto mt-16 hidden  ">
             
         
              <div className="max-w-xs mx-auto sm:max-w-none sm:flex sm:justify-center hidden" data-aos="zoom-y-out" data-aos-delay="300"  >
                <div>
                  <a className="btn text-white bg-blue-600 hover:bg-blue-700 w-full mb-4 sm:w-auto sm:mb-0" href="https://degenhaus.com/getstarted">Get started</a>
                </div>
                <div>
                  <a className="btn text-white bg-gray-900 hover:bg-gray-800 w-full sm:w-auto sm:ml-4" href="https://docs.degenhaus.com">Docs</a>
                </div>
              </div>
             
            </div>
          </div>


          

        

        </div>

      </div>
    </section>
  );
}

export default Auction;