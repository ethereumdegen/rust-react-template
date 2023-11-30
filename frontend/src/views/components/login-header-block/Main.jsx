 
import {useContext} from 'react';

import AccountDropdownMenu from "@/views/components/account-dropdown-menu/Main.jsx";

import { observer } from "mobx-react" 


import {
  Web3StoreContext, 
  SideMenuStoreContext,
  SideBarStoreContext
} from '@/stores/stores-context';




function Main(  ) {

  const sidebarStore = useContext(SideBarStoreContext);
  
  const web3Store = useContext(Web3StoreContext);




  const requireSignIn = false 


  return (
    <>
       <li>


{/*@click="showSidenav()"*/}
{!web3Store.active && 
 <div 

 className="inline-block ml-4 py-2 px-4 text-gray-200 text-xl bg-black hover:text-black hover:bg-gradient-to-r from-blue-500 to-teal-400  cursor-pointer border-l-2 border-gray-100"
   onClick={()=>{ 

     sidebarStore.setOpen(true)


     console.log('open sidebar', sidebarStore)

   }}
   
   >                     
   Connect 
   </div> 
 }

 {web3Store.active && !web3Store.authorized && requireSignIn && 
 <div 

 className="inline-block ml-4 py-2 px-4 text-gray-200  text-xl bg-black hover:text-black hover:bg-gradient-to-r from-blue-500 to-teal-400  cursor-pointer border-l-2 border-gray-100"
   onClick={()=>{ 

     web3Store.requestChallengeAndSign( )

     console.log('request challenge')

   }}
   
   >                     
  Sign In
   </div> 
 }

{web3Store.active && (web3Store.authorized || !requireSignIn) && 
 <AccountDropdownMenu 
 web3Store = {web3Store}
 />
 }
</li>
      {/* END: Icon List */}
    </>
  );
}
 

export default observer(Main);
