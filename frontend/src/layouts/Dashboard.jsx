import { Transition } from "react-transition-group";
import { useState, useEffect } from "react";
import { Link, Outlet, useLocation, useNavigate } from "react-router-dom";

import { linkTo, nestedMenu, enter, leave } from "./index";
import { Lucide } from "@/base-components";

import dom from "@left4code/tw-starter/dist/js/dom";
import SimpleBar from "simplebar";
 
import classnames from "classnames";
import TopBar from "@/views/components/top-bar/Main";
 
import Web3Sidebar from "@/views/components/web3-sidebar/Main.jsx";
 
import { observer } from "mobx-react";
 
 import SideMenu from '@/views/components/side-menu/Main.jsx'
 
 
 
 import {
  Web3StoreContext, 
  SideMenuStoreContext
} from '@/stores/stores-context';
 

function Dashboard() {
  const navigate = useNavigate();
  const location = useLocation();

  const sideMenuStore = useContext(SideMenuStoreContext);
 
 

  return (


    <div className="flex  ">


      <Web3Sidebar 
             
           slot={<div> </div>} 
         
         />  

     
      {/* BEGIN: Side Menu */}
    
      {sideMenuStore.active &&
          <SideMenu 
           
          />
        }
      {/* END: Side Menu */}





      {/* BEGIN: Content */}
      <div
          className="flex-grow"
      >
        <div className="flex-flex-col">
         
        <TopBar

          
        
        />
      </div>


        <div className="content relative">
          <Outlet 
          
           
          />
        </div>
      </div>
      {/* END: Content */}
    </div>
  );
}

export default observer(Dashboard);
