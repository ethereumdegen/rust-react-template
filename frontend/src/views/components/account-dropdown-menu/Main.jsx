import {
    Lucide,
    Dropdown,
    DropdownToggle,
    DropdownMenu,
    DropdownContent,
    DropdownItem,
    DropdownHeader,
    DropdownFooter,
    DropdownDivider,
    
  } from "@/base-components";
 
  
  import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';


import { observer } from "mobx-react" 

import { useNavigate } from 'react-router-dom';

import FrontendConfig from "@/config/frontend-config"



function Main( {sidebarStore, web3Store, sessionStore} ) {

    let navigate = useNavigate();


    return (
        <div                        
        className="ml-4  "
        onClick={()=>{  

            console.log('open account dropdown')

        }}
        
        >                     
      
        <Dropdown>
        <DropdownToggle
        className=" py-2 px-4 text-gray-200 text-xl bg-black hover:text-black hover:bg-gradient-to-r from-blue-500 to-teal-400  cursor-pointer border-l-2 border-gray-100 truncate overflow-ellipsis"
        style={{maxWidth:'140px'}}
        >
            { web3Store.account }
        </DropdownToggle>
        <DropdownMenu className="w-40 text-lg">
            <DropdownContent>

        { FrontendConfig.accountMenu.items.map( (item, index) => (

                    <DropdownItem
                    key={index}
                    onClick={()=>{   
                    
                    navigate(`${item.to}`);
                    }}

                    >{item.label}</DropdownItem>



        ))}

        

            <DropdownItem
        className="text-gray-100 bg-black hover:text-black hover:bg-gradient-to-r from-blue-500 to-teal-400  cursor-pointer border-l-2 border-gray-100 "
        
        onClick={()=>{   
            web3Store.disconnect()
            console.log('disconnect') 
        }}

        >Disconnect</DropdownItem>
            </DropdownContent>
        </DropdownMenu>
        </Dropdown>


        </div> 
    )


}

export default observer(Main);
