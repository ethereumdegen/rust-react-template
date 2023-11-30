import  { useState } from 'react';
 
 

const TableRenderComponent = ({item: render_item}) => {
   
    
      
    // Check for array
    if (Array.isArray(render_item)) {
      return (
        <table className="table">
          <tbody>
            {render_item.map((item, index) => (
              <tr key={index} >
                <td className="p-0">
                  {TableRenderComponent({item})}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      );
    }
    
    // Check for object
    if (typeof render_item === 'object' && render_item !== null) {
      return (
        <table className="table" >
          <thead>
            <tr>
              <th className="border-gray-400 border-2">Key</th>
              <th className="border-gray-400 border-2">Value</th>
            </tr>
          </thead>
          <tbody>
            {Object.entries(render_item).map(([key, item], index) => (
              <tr key={index}   >
                <td   className="border-gray-400 border-2"  >{key}</td>
                <td className="border-gray-400 border-2 p-0"   >
               
                  {TableRenderComponent({item})}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      );
    }
  
    // Check for simple types like string, number, etc.
    return <div>{typeof render_item != 'undefined'  && render_item != null ?  render_item.toString() : "NULL"}</div>;
  
  
  
    };
  
    export default TableRenderComponent;