import { useState, useEffect } from "react";

import { useCallbackState, helper as $h } from "@/utils";
import { Link, Outlet, useLocation, useNavigate } from "react-router-dom";
import { Lucide } from "@/base-components";
 

import classnames from "classnames";
import { observer } from "mobx-react";


import { linkTo, nestedMenu, enter, leave } from "@/utils/sidemenu";
import SimpleBar from "simplebar";

import FrontendConfig from '@/config/frontend-config' 
import DashboardConfig from '@/config/dashboard-config'


import Transition from "../../utils/Transition";



 
import {
  Web3StoreContext, 
  SideMenuStoreContext,
  SideBarStoreContext
} from '@/stores/stores-context';


function SideMenu( {     }) {
  
  const sidebarStore = useContext(SideBarStoreContext);
 
  
  const [formattedMenu, setFormattedMenu] = useState([]); 
  const sideMenu = () => nestedMenu($h.toRaw(DashboardConfig.dashboardMenu), location);
  const [simpleMenu, setSimpleMenu] = useCallbackState({
    active: false,
    hover: false,
    wrapper: false,
  });
  const [mobileMenu, setMobileMenu] = useState(false);

 const navigate = useNavigate()



  useEffect(() => {
    dom("body").removeClass("error-page").removeClass("login").addClass("main");
    new SimpleBar(dom(".side-nav .scrollable")[0]);
    setFormattedMenu(sideMenu());
  }, [sideMenuStore, location.pathname]);


  return (
    

    <nav
        className={classnames({
          "side-nav": true, 
           
          hover: simpleMenu.hover,

          
          "xl:block":true
          
        })}

        style={{
         
          zIndex:"55"
        
        }}
      >  


        <div className="pt-4 mb-4">
          <div className="side-nav__header flex items-center">
            <Link to="/" className="intro-x flex items-center">
              <img
                alt="Rocketman Tailwind HTML Admin Template"
                className="side-nav__header__logo"
                src={FrontendConfig.favicon}
                width="80"
                height="40"
              />
              <span className="side-nav__header__text text-white pt-0.5 text-lg ml-2.5">
               {FrontendConfig.title}
              </span>
            </Link>
            
            <a
              href="#"
              onClick={() => sideMenuStore.toggle()}
              className="  mobile-menu-toggler   ml-auto text-white text-opacity-70 hover:text-opacity-100 transition-all duration-300 ease-in-out pr-5"
            >
              <Lucide icon="XCircle" className="w-5 h-5" />
            </a>
          </div>
        </div>
        <div className="scrollable">
          <ul className="scrollable__content">
            {/* BEGIN: First Child */}
            {formattedMenu.map((menu, menuKey) =>
              typeof menu === "string" ? (
                <li className="side-nav__devider mb-4" key={menu + menuKey}>
                  {menu}
                </li>
              ) : (
                <li key={menu + menuKey}>
                  <a
                    href={menu.subMenu ? "#" : menu.pathname}
                    className={classnames({
                      "truncate": true,
                      "side-menu": true,
                      "side-menu--active": menu.active,
                      "side-menu--open": menu.activeDropdown,
                    })}
                    onClick={(event) => {
                      event.preventDefault();
                      linkTo(menu, navigate);
                      setFormattedMenu($h.toRaw(formattedMenu));
                    }}
                  >
                    <div className="side-menu__icon">
                      <Lucide icon={menu.icon} />
                    </div>
                    <div className="side-menu__title">
                      {menu.title}
                      {menu.subMenu && (
                        <div
                          className={classnames({
                            "side-menu__sub-icon": true,
                            "transform rotate-180": menu.activeDropdown,
                          })}
                        >
                          <Lucide icon="ChevronDown" />
                        </div>
                      )}
                    </div>
                  </a>
                  {/* BEGIN: Second Child */}
                  {menu.subMenu && (
                    <Transition
                      in={menu.activeDropdown}
                      onEnter={enter}
                      onExit={leave}
                      timeout={300}
                    >
                      <ul
                        className={classnames({
                          "side-menu__sub-open": menu.activeDropdown,
                        })}
                      >
                        {menu.subMenu.map((subMenu, subMenuKey) => (
                          <li key={subMenuKey}>
                            <a
                              href={subMenu.subMenu ? "#" : subMenu.pathname}
                              className={classnames({
                                "side-menu": true,
                                "side-menu--active": subMenu.active,
                              })}
                              onClick={(event) => {
                                event.preventDefault();
                                linkTo(subMenu, navigate);
                                setFormattedMenu($h.toRaw(formattedMenu));
                              }}
                            >
                              <div className="side-menu__icon">
                                <Lucide icon="Activity" />
                              </div>
                              <div className="side-menu__title">
                                {subMenu.title}
                                {subMenu.subMenu && (
                                  <div
                                    className={classnames({
                                      "side-menu__sub-icon": true,
                                      "transform rotate-180":
                                        subMenu.activeDropdown,
                                    })}
                                  >
                                    <Lucide icon="ChevronDown" />
                                  </div>
                                )}
                              </div>
                            </a>
                            {/* BEGIN: Third Child */}
                            {subMenu.subMenu && (
                              <Transition
                                in={subMenu.activeDropdown}
                                onEnter={enter}
                                onExit={leave}
                                timeout={300}
                              >
                                <ul
                                  className={classnames({
                                    "side-menu__sub-open":
                                      subMenu.activeDropdown,
                                  })}
                                >
                                  {subMenu.subMenu.map(
                                    (lastSubMenu, lastSubMenuKey) => (
                                      <li key={lastSubMenuKey}>
                                        <a
                                          href={
                                            lastSubMenu.subMenu
                                              ? "#"
                                              : lastSubMenu.pathname
                                          }
                                          className={classnames({
                                            "side-menu": true,
                                            "side-menu--active":
                                              lastSubMenu.active,
                                          })}
                                          onClick={(event) => {
                                            event.preventDefault();
                                            linkTo(lastSubMenu, navigate);
                                          }}
                                        >
                                          <div className="side-menu__icon">
                                            <Lucide icon="Zap" />
                                          </div>
                                          <div className="side-menu__title">
                                            {lastSubMenu.title}
                                          </div>
                                        </a>
                                      </li>
                                    )
                                  )}
                                </ul>
                              </Transition>
                            )}
                            {/* END: Third Child */}
                          </li>
                        ))}
                      </ul>
                    </Transition>
                  )}
                  {/* END: Second Child */}
                </li>
              )
            )}
            {/* END: First Child */}
          </ul>
        </div>
      </nav>


       

  );
}
 

export default observer(SideMenu);
