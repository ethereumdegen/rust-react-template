import {useContext , useEffect} from 'react';
 
 
import { observer } from "mobx-react";

import { useLocation, useNavigate } from 'react-router-dom';


import {
  Web3StoreContext, 
  ChatStoreContext,
  SideBarStoreContext
} from '@/stores/stores-context';

 
function Home() {
    const location = useLocation();
    const navigate = useNavigate();

  const web3Store = useContext(Web3StoreContext);
 
  const chatStore = useContext(ChatStoreContext);
   


  useEffect(() => {
    const query = new URLSearchParams(location.search);
    const accessToken = query.get('access_token'); 
    const accessTokenDomain = query.get('access_token_domain'); 

    console.log( {accessToken} );


    chatStore.connect(  {
        appName: accessTokenDomain,
        appAuthToken: accessToken
    }  )


    //redirect to chat 

    navigate("/chat")

  }, [location]);

 

  return (
    <div className="flex flex-col min-h-screen overflow-hidden">
 
      
      <main className="flex-grow">
 

        

      <div  >
 

        Oauth Complete  
 

      </div>


      </main>
 

     

    </div>
  );
}

export default observer(Home);