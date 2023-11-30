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


import ConfirmButton from "./components/confirm_button.jsx"
import Modal from "@/views/components/modal/Main";

import SignInRequiredWarning from "@/views/components/sign-in-required-warning/Main";
import SimpleButton from "@/views/components/button/SimpleButton";
 
import {
  Web3StoreContext, 
  SideMenuStoreContext,
  SideBarStoreContext,
  ChatStoreContext,
} from '@/stores/stores-context';

import Chat from "./components/chat.jsx"

import Connect from "./components/connect.jsx"

function Main() {
  

  const web3Store = useContext(Web3StoreContext);
  const sidebarStore = useContext(SideBarStoreContext);
  const chatStore = useContext(ChatStoreContext);
   
 
 
   
 
  if (chatStore.connected) {



    return (
      <Chat /> 
    );
  

  }else {


  return (
    <Connect /> 
  );

  }
  
 



}

export default observer(Main);
