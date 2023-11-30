// TableWithPagination.js
import React, { useState , useEffect} from 'react';
import PaginationBar from './PaginationBar';



import DefaultTableRow from './rows/GenericTableRow';

/*

fetchRows is a callback that calls the API to fetch the rows  -- accepts args (currentPage, rowsPerPage)
*/
const TablePaginated = ({ headers, rowsPerPage , fetchRows , tableRowComponent , forceUpdate}) => {
  const [currentPage, setCurrentPage] = useState(1);

  const [rows, setRows] = useState([]);

  const totalPages = () => {

     return Math.ceil(rows.length / rowsPerPage);
  } 

  /*const paginateRows = (rows) => {
    const startIndex = (currentPage - 1) * rowsPerPage;
    const endIndex = startIndex + rowsPerPage;
    return rows.slice(startIndex, endIndex);
  };*/

  const updateRows = async () => {

    let newRows = []
    let fetchRowsResponse = await fetchRows(currentPage, rowsPerPage);

    console.log({fetchRowsResponse})

    if(fetchRowsResponse && Array.isArray(fetchRowsResponse)){
        newRows = fetchRowsResponse
    }

    setRows(newRows);
  }




  /*  //load  on mount 
    useEffect(()=>{
        fetchRows()
    }, []) // <-- empty dependency array
*/


     // Add useEffect to listen for changes in the forceUpdate prop
  useEffect(() => {
    updateRows()
  }, [forceUpdate]);



// Use tableRowClass if it's provided, otherwise use the fallback component
const TableRow = tableRowComponent ? tableRowComponent :  DefaultTableRow;
  

  //onMount run onPageUpdated -> fetchRows -> setRows 

  return (
    <div>
      <table className="w-full table-auto">
        <thead>
          <tr className="text-left bg-gray-200">
            {headers.map((header, index) => (
              <th key={index} className="px-4 py-2">
                {header.displayName}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {rows.map((row, rowIndex) => (

        React.createElement(TableRow, { 
            key: rowIndex,
            rowIndex:rowIndex,
            row: row 
        })

           
            
          ))}

          {!rows || rows.length === 0 && <tr>
            <td colSpan={headers.length} className="text-center py-4">No rows to display</td>
            </tr>}
        </tbody>
      </table>
    
        {rows && rows.length > 0 && 
            <PaginationBar
            currentPage={currentPage}
            totalPages={totalPages()}
            onPageChange={(pageNumber) => {
                setCurrentPage(pageNumber)
                fetchRows() 
            }}
            />
        }
    
    </div>
  );
};

export default TablePaginated;
