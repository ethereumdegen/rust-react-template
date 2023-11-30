


function Main({row, rowIndex}) {

return (
    
    <tr
         
        className={rowIndex % 2 === 0 ? 'bg-gray-100' : ''}
    >
        {row && row.map((cell, cellIndex) => (
        <td key={cellIndex} className="px-4 py-2 border">
            {cell}
        </td>
        ))}
    </tr>
    
)


}


export default Main;
