import React,{ useState , useRef, useEffect} from 'react';
 
import { Link,useNavigate } from 'react-router-dom';

 
import {observer} from "mobx-react"

 
 
import FrontendConfig from '@/config/frontend-config'
 
 


function HeaderSidebar({slot,headerStore })   {
 
       

  let navigate = useNavigate();


  return ( 
   <>
    { headerStore.mobileNavOpen &&   (
 
  <div
  
  className=" animate__animated animate__slideInLeft fixed left-0 top-0 p-6 h-full bg-neutral-800 flex flex-col "

  style={{zIndex:"55", minWidth:"240px"}}
  >


    <div className="flex flex-row">

    
    {/* Logo */}
    <div>
            <Link to="/" className="intro-x flex items-center ">
              <img
                style={{ height:'40px', width:'80px' }}
                className="side-nav__header__logo"
                src={FrontendConfig.favicon}
              
              />
              <span className="side-nav__header__text text-black pt-0.5 text-lg ml-2.5">
            
              </span>
            </Link>
            </div>
    

         <div 
      className="w-full text-white flex justify-end cursor-pointer mb-8"
      onClick={()=> headerStore.setMobileNavOpen(false)}
      > 
       <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fillRule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clipRule="evenodd"></path></svg> 
      </div>
    </div>
   


    <div className="flex-grow">

      
    
 
        <div className= "flex flex-col">
            {FrontendConfig.navbar.items.map((item, index) => (
              <div className='p-1 my-1'>

            <Link 
             key={index} 
            to={item.to ? item.to : item.href }  
            onClick={()=>{ 
              headerStore.setMobileNavOpen(false)
             
            }}
            className='p-2 cursor-pointer text-xl text-gray-100 hover:bg-gray-800' key={index} >
            {item.label} 
            </Link>
          
                
          
              </div>
            ))}
            </div>


      </div>



       



      </div>
      


    
    )}  </>
              
      
      
  );
}

export default observer(HeaderSidebar);

 