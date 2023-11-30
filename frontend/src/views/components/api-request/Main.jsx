import React, { useState } from 'react';
import axios from 'axios';

import {getBackendServerUrl} from '@/lib/app-helper'

function ApiRequestComponent({endpoint, type, input}) {

  
  let [inputValue, setInputValue] = useState('');
  let [outputResponse, setOutputResponse] = useState('');

  

  const backendServerUrl = "https://goerli.ensapi.io:8443"
  const endpointURL = `${backendServerUrl}${endpoint}`

    //initialize
  if(input && !inputValue) {inputValue = JSON.stringify(input);}
  

  const handleSubmit = async (event) => {

    
    event.preventDefault();
    try { 

      setOutputResponse( '' )

      const data = JSON.parse(inputValue)
      const response = await axios.post(endpointURL,   data );
   
      if(response.data) {
        if(response.data.success){
          setOutputResponse(JSON.stringify(response.data))
        }else{
          setOutputResponse(JSON.stringify(response.data))
        }
      } 
     

    } catch (error) {
      console.error('Error making POST request:', error);
      setOutputResponse( 'Error making POST request' )
    }
  };

   const handleInputChange = (event) => {
     
    setInputValue(event.target.value);
  }; 

  return (
    <form
     onSubmit={handleSubmit}
     className=""
     style={{maxWidth:"240px"}}
    >
          <span className="font-bold my-2"> Try it out </span>  

      <div>

      <div className="my-4"> POST { endpointURL } </div>
        
        <textarea 
        type="text"
        className="bg-transparent my-2 text-sm"
        placeholder="Enter data to POST"
        rows="5"
        defaultValue={inputValue} 
        onChange={handleInputChange} 
         
         />
      </div>

    <div className="text-right p-2 m-2">
        <button 
        className="bg-blue-500 text-black p-2 rounded hover:bg-blue-300"
        type="submit"
        >Submit</button>

    </div>


    <div>
        {outputResponse && <div className="overflow-x-auto">
                <span className="font-bold"> Response </span>  
                <div className="p-2  break-all bg-gray-300 text-black" style={{maxWidth:"450px"}}> {outputResponse} </div>
        </div>}
    </div>

    </form>
  );
};

export default ApiRequestComponent;
