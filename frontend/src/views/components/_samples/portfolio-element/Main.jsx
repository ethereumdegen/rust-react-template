 
 import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
 

import { observer } from "mobx-react";

function Main({title, image, description, link }) {

  const [hovered, setHovered] = useState(false);

  let navigate = useNavigate();


  let titleColor = hovered? 'text-green-200' : 'text-white'
 
 
  return (
    <div className="flex flex-col border-2 border-gray-200 rounded text-center relative"  > 

                   
    <div className="h-full relative " 
    
    onMouseEnter={() => setHovered(true)}
    onMouseLeave={() => setHovered(false)}
    
    >


      
      <a href={link} >
        <img  className="my-auto" src={`${image}`}/>
      </a>


    </div>

    <div 
    className={`bg-gray-800 ${titleColor} py-2 font-bold `}
    
    > {`${title}`} </div>

    {hovered && 
        <div className="absolute h-full w-full bg-black  pointer-events-none  bg-opacity-50  p-8">
        <div className=" text-white text-xl">{`${description}`}</div>
        </div>
    }

  </div>
  )

}

  


export default observer(Main);


