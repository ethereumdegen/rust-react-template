import { useRoutes } from "react-router-dom";
import MainLayout from "../layouts/Main";


import ContextLayout from "../layouts/Context"
 
 
import Welcome from '../views/welcome/Main'
     
 
 
import ErrorPage from "../views/error-page/Main";

import Chat from "../views/chat/Main" 
import OAuthComplete from "../views/system/OAuthComplete"
    
function Router() {
  const routes = [

    {
      element:<ContextLayout /> ,
    children: [


      {
      
        element: <MainLayout />,
        children:  [
          
          
          {
            path:"/",
            element: <Welcome />, 
          },
   
  
            
          {
            path: "/chat",
            element: <Chat />,
          },

          {
            path: "/oauth/complete",
            element: <OAuthComplete />,
          },
        
 
 

        ]
      
    },
     
    
      {
        path: "/error-page",
        element: <ErrorPage />,
      },
      {
        path: "*",
        element: <ErrorPage />,
      },


    ]
    }
    
  ];

  return useRoutes(routes);
}

export default Router;
