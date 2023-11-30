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


import ConfirmButton from "./confirm_button.jsx"
import Modal from "@/views/components/modal/Main";

import SignInRequiredWarning from "@/views/components/sign-in-required-warning/Main";
import SimpleButton from "@/views/components/button/SimpleButton";
 
import {
  Web3StoreContext, 
  ChatStoreContext,
  SideBarStoreContext
} from '@/stores/stores-context';


import { BigNumber, Contract, ethers, utils } from "ethers";

import AlertBanner from "@/views/components/alert-banner/Main";
import { filterEndpointOutput } from "../../../utils/chat-render.js";
 
import TableRenderComponent from "./table-render.jsx";

import FlexibleRenderComponent from "./flexible-render.jsx";


function Main() {
  

  const web3Store = useContext(Web3StoreContext);
  const chatStore = useContext(ChatStoreContext);
   
  
  const [errorMessage, setErrorMessage] = useState(null);
  
  const [chatHistoryRows, setChatHistoryRows] = useState([ ]);
 
  const [inputText, setInputText] = useState('');
  


  const StandardChatMessageBody = ({ message }) => {
    return (
      <div>
        <p>{message}</p>
        {/* Additional JSX here, based on 'additionalData' or whatever you need */}
      </div>
    );
  };


 


 const StandardCommandMessageBody = ({ command_name, command_arguments }) => {
    return (
      <div className="p-2">
        <div className="p-2 font-bold text-lg mb-2 " >{command_name}</div>

       
       
        <div className="p-2">


      {  TableRenderComponent({item:command_arguments})}


        
        </div>



        {/* Additional JSX here, based on 'additionalData' or whatever you need */}
      </div>
    );
  };

  
  const performButtonAction = async (button_action_data) => {
    
      console.log("performing button action ", button_action_data)
     
     //hit the backend to start the challenge generation process !  this will also store a record in the db .
      let auth_data = {
        access_token:  chatStore.connectedAppAuthToken  ,
        access_domain:  chatStore.connectedAppName  ,

      }

      let commandData =  { ...button_action_data , ...auth_data }
     
      let response = await backendCallForRoute({
          methodName:'confirm_command',
          endpointParams:{


              // need auth token here 
              command: commandData
          
          
           
             // auth_token: web3Store.authToken   //add me later 
             

          }
      })
      
      
       if (response?.status != 200 ){
        console.log("ERROR ",{response})
        return 
      }
      
      let response_data = response?.data;
      
      console.log({response_data})

      if( response_data  ){
        let endpoint_data = response_data?.endpoint_data ; 
        let command_name = response_data?.command_name;

        let filtered_endpoint_data = filterEndpointOutput(command_name, endpoint_data  )

       //   let response_data_chat_text = JSON.stringify( endpoint_data  )

          addChatHistoryRow({
            from: "data", 
           // body: response_data.message.toString(),
             body: FlexibleRenderComponent ( {item:filtered_endpoint_data} ),

            action_buttons: [ 
              
            ]
            
            })
          




         // addChatHistoryRow({from: "data", body:  response_data_chat_text})
   



        return true 
        //hide the btn 
      }
      
       // redirect the user to the new URL in a new tab
       /* if (response_data) {
          window.open(response_data, '_blank');
        }*/
     
  }
  
  const addChatHistoryRow = ({from, body, action_buttons}) => {
    
    setChatHistoryRows((prevRows) => [
        ...prevRows,
        {
          from,
          body,
          action_buttons,
        },
      ]);
    
  }

  const handleSubmit = async (e) => {
    e.preventDefault(); // Prevent the default form submission behavior
    
    let input_text_cached = `${inputText}`  

    setInputText(''); // Clear the input field

    addChatHistoryRow({from: "user", body: input_text_cached})
   
    let response = await backendCallForRoute({
          methodName:'chat_input',
          endpointParams:{
           
              body:  input_text_cached,
             // auth_token: web3Store.authToken   //add me later 
             

          }
      })
       
      if (response?.status != 200 ){
        console.log("ERROR ",{response})
        return 
      }
        
      //add chat text?? if there is a fn call for twitter connect, can add a button to do so right in the chat . 
    console.log({response})
      
    let response_data = response?.data;
      
    if( response_data.command  ){
      
      let command_name = response_data.command.command_name;
      let command_arguments = response_data.command.arguments;


      
          addChatHistoryRow({
            from: "agent", 
         //   body: "It seems like you want to sign in with twitter.",

            body: StandardCommandMessageBody ({command_name,command_arguments}),

            action_buttons: [
                
                { label:"Confirm this command", action: { command_name , command_arguments }}
              
            ]
            
            })
          
          //add a chat prompt with a button that will let the user connect twitter oauth! 
          
      
    }
      
    if( response_data.message ){
        
       addChatHistoryRow({
            from: "agent", 
           // body: response_data.message.toString(),
             body: StandardChatMessageBody ({message: response_data.message.toString()}),

            action_buttons: [ 
              
            ]
            
            })
          

        //need to also present some metadata about the record that is going to be inserted .. but yeah
    }
      
      
   
  };
 

  const navigate = useNavigate();

  const handleKeyPress = (e) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSubmit(e);
    }
  };
 
 
  observe(web3Store, "account", function () {
    console.log("acct:", web3Store.account);
  });

  observe(web3Store, "authorized", function () {
    console.log("acct:", web3Store.account);
  });

  
 
  return (
    <>


 


      <div className="intro-y flex flex-col sm:flex-row items-center mt-2"></div>
      <div className="intro-y box pt-4 px-5 pb-4 mt-2 flex flex-col items-center">
      
        
      
      
        <div className="pt-4 px-2 pb-16 w-full">
          {/* BEGIN:   Title */}

          <div className=" mt-2 mb-5 ">
            <div className="text-md text-center   my-2 ">
                { chatStore.connectedAppName &&  
                  !chatHistoryRows?.length > 0 && 
                   <span className="capitalize text-gray-400"> Connected to {chatStore.connectedAppName} with Artificial Intelligence </span>
                }     
            </div>
          </div>

          {/* END: Tx Title */}
          {/* BEGIN: Tx Content */}

          <div className="w-full">
            <div className="flex flex-col">
              <div className="px-4 mb-4 text-lg font-bold"></div>

               <div>

               <form onSubmit={handleSubmit}>
                    <div className="mb-4">
                    {/* Your chat messages can go here */}
                    
                    
                    
                    <ul>
                  {chatHistoryRows.map((row, index) => (
                    <li key={index} className="bg-gray-200 p-4 rounded-lg mb-2 flex flex-col">
                      <div className="mb-2 flex flex-row">
                        <div className=" ">
                        <div className={`inline-block text-white rounded font-bold p-2 
                        capitalize ${row.from === 'agent' ? 'bg-purple-400' : 'bg-green-500'}`}>

      
                            {row.from}
                            </div>
                          </div> 
                          
                          <div className="p-2 flex-grow ">{row.body}</div> 
                      </div>
                      <div className="flex space-x-2 text-center ">

                        {row.action_buttons && row.action_buttons.map((button, i) => (
                         
                         <ConfirmButton 
                         key={i}
                         label={button.label}
                         action={button.action}
                         performButtonAction={performButtonAction}
                         />
                         
                       
                        ))}
                      </div>
                    </li>
                  ))}
                </ul>

                    
                    
                    </div>
                    
                    <div className="flex">
                    <textarea
                        className="flex-grow p-2 border rounded"
                        placeholder="Type your message..."
                        value={inputText}
                        onChange={(e) => setInputText(e.target.value)}
                        onKeyPress={handleKeyPress}
                        rows={2} // Optional: Sets the initial number of rows
                    />
                    <button
                        type="submit"
                        className="ml-2 px-4 py-2 text-white bg-blue-500 hover:bg-blue-400 rounded"
                    >
                        Submit
                    </button>
                    </div>
                </form>


               </div>
            </div>
          </div>

          <div className="w-full text-center">

                 <div 
                 className="px-4 mt-32 cursor-pointer border-2 border-gray-400 inline-block"
                 onClick={()=>{
                    console.log("disconnect")
                    chatStore.disconnect()

                 }}
                 >
                        Disconnect
                </div>           

          </div>

          {/* END: Tx Content */}
        </div>
      </div>
    </>
  );
}

export default observer(Main);
