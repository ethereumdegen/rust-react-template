import {useContext} from 'react';

import HeroHome from '../partials/HeroHome';
 
import { observer } from "mobx-react";

import {
  Web3StoreContext, 
  SideMenuStoreContext,
  SideBarStoreContext
} from '@/stores/stores-context';

 
function Home() {


  const web3Store = useContext(Web3StoreContext);
 


  return (
    <div className="flex flex-col min-h-screen overflow-hidden">
 
      {/*  Page content */}
      <main className="flex-grow">

        {/*  Page sections */}
        <HeroHome />

        {/*<FeatureCode />*/}

      <div>
 
        
 

      </div>


      </main>
 

     

    </div>
  );
}

export default observer(Home);