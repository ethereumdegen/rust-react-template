import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';

import LoginHeaderBlock from "@/views/components/login-header-block/Main.jsx";
 
import FrontendConfig from '@/config/frontend-config'
 
 
 
import { observer } from "mobx-react" 


function Header( {sidebarStore, web3Store, sessionStore, headerStore} ) {

  const [top, setTop] = useState(true);


  // detect whether user has scrolled the page down by 10px 
  useEffect(() => {
    const scrollHandler = () => {
      window.pageYOffset > 10 ? setTop(false) : setTop(true)
    };
    window.addEventListener('scroll', scrollHandler);
    return () => window.removeEventListener('scroll', scrollHandler);
  }, [top]);  

  return (
    <header className={` w-full z-30 md:bg-opacity-90 transition duration-300 ease-in-out ${!top && 'bg-white backdrop-blur-sm shadow-lg'}`}>
      <div className="max-w-6xl mx-auto px-5 sm:px-6">
        <div className="flex items-center justify-between h-16 md:h-20">

          {/* Site branding */}
          <div className="flex  mr-4 flex-row">


              <div 
              className="xl:hidden cursor-pointer" 
              onClick={()=>{ headerStore.toggleMobileNav() }}
              >
                <svg width="30" height="30" viewBox="0 0 30 30" aria-hidden="true"><path stroke="currentColor" strokeLinecap="round" strokeMiterlimit="10" strokeWidth="2" d="M4 7h22M4 15h22M4 23h22"></path>
                
                </svg></div>



            {/* Logo */}
            <div>
            <Link to="/" className="intro-x flex items-center ml-4">
              <img
                style={{ height:'40px', width:'80px' }}
                className="side-nav__header__logo"
                src={FrontendConfig.favicon}
              />
              <span className="side-nav__header__text text-black pt-0.5 text-lg ml-2.5">
            
              </span>
            </Link>
            </div>

            <div className="hidden xl:block">
            {FrontendConfig.navbar.items.map((item, index) => (

              <Link to={item.to ? item.to : item.href }   className='p-4 text-lg' key={index} > {item.label} </Link>
          
          ))}
            </div>


          </div>

          {/* Site navigation */}
          <nav className="flex flex-grow">
            <ul className="flex flex-grow justify-end flex-wrap items-center">


              <LoginHeaderBlock
                web3Store={web3Store}
                sidebarStore={sidebarStore}
              
              ></LoginHeaderBlock>


             
               
            </ul>

          </nav>

        </div>
      </div>
    </header>
  );
}

export default observer(Header);
