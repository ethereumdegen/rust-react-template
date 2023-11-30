 
import { useState, useEffect } from "react";
import { Link, Outlet, useLocation, useNavigate } from "react-router-dom";
 


 
import { Web3SidebarStore } from '@/stores/web3-side-bar.js';
import { Web3Store } from '@/stores/web3-store-mobx.js';
   
import { SideMenuStore } from '@/stores/side-menu-mobx.js';

import { ChatStore } from '@/stores/chat-store-mobx.js';
 
import { 
    Web3StoreContext,
    ChatStoreContext,
     
      SideBarStoreContext,
        
       SideMenuStoreContext,
    } from '@/stores/stores-context';

const sideMenuStore = new SideMenuStore()
const sidebarStore = new Web3SidebarStore()
const web3Store = new Web3Store()  
const chatStore = new ChatStore()  

function Main() {
   

  return (
    <div className=" ">
         
         <Web3StoreContext.Provider value={web3Store}>
           <ChatStoreContext.Provider value={chatStore}>
              <SideMenuStoreContext.Provider value={sideMenuStore}>
                <SideBarStoreContext.Provider value={sidebarStore}>
                       
                       
                         <Outlet 
                        
                            />

             
                </SideBarStoreContext.Provider>
               </SideMenuStoreContext.Provider>
             </ChatStoreContext.Provider>
           </Web3StoreContext.Provider>
 
     
    </div>
  );
}

export default Main;
