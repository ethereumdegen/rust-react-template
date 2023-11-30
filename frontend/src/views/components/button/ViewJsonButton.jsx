 

function Main({customClass, children, jsonData}) {
  
    const jsonString = JSON.stringify(jsonData);
    const base64Json = btoa(jsonString);
    const dataUrl = `data:application/json;base64,${base64Json}`;

/*
    function jsonToDataUrl(jsonData) {
        const jsonString = JSON.stringify(jsonData);
        const base64Json = btoa(jsonString);
        return `data:application/json;base64,${base64Json}`;
     }
     */


    return (
      <a
        className= {`p-2 mx-2 text-md border-2 border-slate-400 rounded select-none cursor-pointer ${  customClass }`}
        href={dataUrl}
      >
  
        {children}
      </a>
  
    );
  }
   
  
  export default Main;
  