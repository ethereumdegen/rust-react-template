// src/component/Page
import type { ReactNode } from "react";
import React from "react";

import FrontendConfig from '@/config/frontend-config'
import DocsConfig from '@/config/docs-config'

import DocsSidenav from '@/views/components/docs-sidenav/Main.jsx'

import ApiRequestComponent from '@/views/components/api-request/Main.jsx'
import { useLocation } from 'react-router-dom'

interface Props {
  attributes: Record<string, any>;
  children: ReactNode;
}
// props will contain attributes
function Page(props: Props) {

  const location = useLocation()
  console.log(location)
    
  let pathname = location.pathname

  let requestComponentConfig = DocsConfig.pages[pathname]

 

  const { children, attributes } = props;
  return (
    <React.Fragment>
      <h1>{attributes.name}</h1>

      {/* sidebar nav here ! for the markdown pages .. from config  */}

     <div className="  mx-auto my-2   flex flex-row mb-16 ">
        
        <div className="   border-2 border-gray-200">
          <DocsSidenav  />
          </div>

         <div className="markdown-body overflow-x-scroll px-8 py-8 flex-grow border-t-2 border-r-2 border-b-2 border-gray-200 ">
           {children}
          </div>

    { requestComponentConfig && requestComponentConfig.apiRequestComponent &&  <div 
         
          className=" border-2 border-gray-200 hidden lg:flex bg-black text-gray-200">

            <div className="p-4">
            <ApiRequestComponent

              endpoint={requestComponentConfig.apiRequestComponent.endpoint}
              type={requestComponentConfig.apiRequestComponent.type}
              input={requestComponentConfig.apiRequestComponent.input}

            
            />
            </div>

          </div>

    }
     
    
      </div> 

    </React.Fragment>
  );
}
export default Page;