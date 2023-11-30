 

function Main( {authorized, children}) {

    return (
      
    <>
    {!authorized  && 
      
    <div className="px-4 py-16 text-lg font-bold">

     {children}
    
    </div>

    }
    </>
    );
  }
  
  export default Main;
  