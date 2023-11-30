
function getTypeNameFromCommandName(commandName){

    if(commandName ==  "FindProducts"){
        return "ProductComplex"
    }

    if(commandName ==  "FindProduct"){
        return "ProductComplex"
    }

    return undefined 

}

export function filterEndpointOutput( 
    commandName,
    rows
 ){ 
    if(rows == null ) return null ;

    console.log("ROWS ARE ", rows )

    if(rows.rows != null && Array.isArray(rows.rows)){
       
        rows =   rows.rows  ;
    }

    if(!Array.isArray(rows)){
        rows = [rows]
    }

    let filterableTypeName = getTypeNameFromCommandName( commandName ) 
 
    console.log({commandName,  filterableTypeName })
    if (filterableTypeName == "ProductComplex"){

       

        let only_props = undefined

        if(rows.length >=2){
            only_props = ["id","sku","title", "enabled"]
        }

        console.log( rows.map((row) => row.product )  )

        if( only_props ){
            return rows.map((row) => {
                return Object.keys(row.product).reduce((newObj, key) => {
                    if (only_props.includes(key)) {
                        newObj[key] = row.product[key];
                    }
                    return newObj;
                }, {});
            });
        }else{
            return rows.map((row) => row.product ) 
        }
      

    }



    return rows 

}