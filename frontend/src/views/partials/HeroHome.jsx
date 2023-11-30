import React, { useState } from 'react';
import Modal from '../utils/Modal';

 
import FrontendConfig from '@/config/frontend-config'
 

function HeroHome() {

  const [videoModalOpen, setVideoModalOpen] = useState(false);

  return (
    <section className="relative">

      {/* Illustration behind hero content */}
      <div className="absolute left-1/2 transform -translate-x-1/2 bottom-0 pointer-events-none" aria-hidden="true">
        <svg width="1360" height="578" viewBox="0 0 1360 578" xmlns="http://www.w3.org/2000/svg">
          <defs>
            <linearGradient x1="50%" y1="0%" x2="50%" y2="100%" id="illustration-01">
              <stop stopColor="#FFF" offset="0%" />
              <stop stopColor="#EAEAEA" offset="77.402%" />
              <stop stopColor="#DFDFDF" offset="100%" />
            </linearGradient>
          </defs>
          
        </svg>
      </div>

      <div className="max-w-6xl mx-auto px-4 sm:px-6">

        {/* Hero content */}
        <div className="pt-32 pb-12 md:pt-40 md:pb-20 ">

          {/* Section header */}
          <div className="text-left pb-12 md:pb-16 relative">

          <img className="select-none" src={ `${FrontendConfig.homeImage}` } ></img>

            <div className="absolute top-4 left-16 md:top-32 md:left-24 text-bold text-2xl md:text-4xl "> 
                {FrontendConfig.title}
                </div>
           
            <div className=" w-full ">
              <p className=" text-lg md:text-2xl text-gray-600 mb-8 my-8 text-right" data-aos="zoom-y-out" data-aos-delay="150">

         

               {FrontendConfig.tagline}
                
                </p>

              </div>
              <div className="max-w-3xl mx-auto mt-16 hidden  ">
             
         
              <div className="max-w-xs mx-auto sm:max-w-none sm:flex sm:justify-center hidden" data-aos="zoom-y-out" data-aos-delay="300"  >
                <div>
                  <a className="btn text-white bg-blue-600 hover:bg-blue-700 w-full mb-4 sm:w-auto sm:mb-0" href="https://degenhaus.com/getstarted">Get started</a>
                </div>
                <div>
                  <a className="btn text-white bg-gray-900 hover:bg-gray-800 w-full sm:w-auto sm:ml-4" href="https://docs.degenhaus.com">Docs</a>
                </div>
              </div>
             
            </div>
          </div>


          

        

        </div>

      </div>
    </section>
  );
}

export default HeroHome;