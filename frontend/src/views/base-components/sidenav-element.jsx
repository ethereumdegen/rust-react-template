 
import { Link } from 'react-router-dom';
function Main( {label, to} ) {
    
 
 
 
  return (
    <>
         <Link to={to} className="hover:bg-gray-300 rounded py-1 px-2 my-1"  >
          
          {label}

         </Link>
  
    </>
  );
}

export default  (Main);
