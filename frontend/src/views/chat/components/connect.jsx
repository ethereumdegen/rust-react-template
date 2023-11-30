import { useState, useEffect, useContext } from "react";

import {
  useOutletContext,
  useParams,
  useSearchParams,
  useNavigate,
} from "react-router-dom";


import {
  Lucide,
  
} from "@/base-components";

import axios from 'axios'

import { observer } from "mobx-react";
import { observe } from "mobx";


import {backendCallForRoute} from '@/utils/frontend-helper.ts'
 

import SignInRequiredWarning from "@/views/components/sign-in-required-warning/Main";
import SimpleButton from "@/views/components/button/SimpleButton";
 
import {
  Web3StoreContext, 
  ChatStoreContext,
  SideBarStoreContext
} from '@/stores/stores-context';


import { BigNumber, Contract, ethers, utils } from "ethers";

import AlertBanner from "@/views/components/alert-banner/Main";
 

function Main() {
  

  const web3Store = useContext(Web3StoreContext);
  const sidebarStore = useContext(SideBarStoreContext);
   
  const chatStore = useContext(ChatStoreContext);
   
  
  const [errorMessage, setErrorMessage] = useState(null);
   


  useEffect(()=> {

    chatStore.loadState()


  },[])

  const ConnectButton = ({ label }) => {
    return (
     
      <a 
      className="p-8 block select-none hover:bg-blue-200 cursor-pointer border-2 border-gray-500 rounded"
      onClick={ async () => {  


        //This will actually go into system oauth complete
        
        let response = await backendCallForRoute({
            methodName:'oauth_indiefuture_init',
            endpointParams:{
            
                //public_address: web3Store.account,
                //auth_token: web3Store.authToken,
                

            }
        })

        if(response.status == 200){

          let response_data = response?.data;

          let auth_url = response_data ;

          window.location = auth_url ;

            console.log({response})

        }else{
          console.log({response})
        }




        } }
      >
         {label} 
        {/* Additional JSX here, based on 'additionalData' or whatever you need */}
      </a>
    
    );
  };

  
 
 

  const navigate = useNavigate();

 
 
  observe(web3Store, "account", function () {
    console.log("acct:", web3Store.account);
  });

  observe(web3Store, "authorized", function () {
    console.log("acct:", web3Store.account);
  });

  

  const connectButtons = [
    {label: "IndieFuture"}

  ]
 
  return (
    <>


 


      <div className="intro-y flex flex-col sm:flex-row items-center mt-2"></div>
      <div className="intro-y box pt-4 px-5 pb-4 mt-2 flex flex-col items-center">
      
        
      
      
        <div className="pt-4 px-2 pb-16 w-full lg:w-3/4 mx-auto">
          {/* BEGIN:   Title */}

          <div className=" mt-2 mb-5 ">
            <div className="text-xl   my-2 ">
            Connect to an Application

            </div>
          </div>

          {/* END: Tx Title */}
          {/* BEGIN: Tx Content */}

          <div className="w-full">
            <div className="flex flex-col">
             

               
             <div className="mb-4">
                   
                     
                    
                    
                    <div>
                    {connectButtons.map((item, index) => (
                        <ConnectButton
                            key={index}
                            label={item.label}
                        />  
                    ))}
                    </div>

                     


               </div>
            </div>
          </div>

          {/* END: Tx Content */}
        </div>
      </div>
    </>
  );
}

export default observer(Main);
