 

import makeBlockie from 'ethereum-blockies-base64';
 

import { observer } from "mobx-react" 


function Main( {web3Store} ) {
    

 let imageSource
 let width = 64 
 

 if(web3Store.account){
    console.log('making blockie', web3Store)
     imageSource = makeBlockie(web3Store.account);

 } 

 
  return (
    <>
         { (imageSource&&width) && <img src={imageSource} width={width} /> }
  
    </>
  );
}

export default observer(Main);
