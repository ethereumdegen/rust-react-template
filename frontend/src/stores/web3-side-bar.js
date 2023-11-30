 
//https://mobx.js.org/the-gist-of-mobx.html
 
import { makeObservable,makeAutoObservable, observable, action } from "mobx"

export class Web3SidebarStore {
     
    open=false 
    addressCopiedAlertEnabled=false

    addressCopiedTimeout=undefined 
  
    constructor() {

    //  makeAutoObservable(this)
      
      
        makeObservable(this, {
            open: observable ,
            addressCopiedAlertEnabled: observable
        })



        
    }

    setOpen(open) {
        this.open = open
    }

    ///do based on a timer ? 
    triggerAddressCopiedAlert() {
        this.addressCopiedAlertEnabled = true


        clearInterval(this.addressCopiedTimeout)
        this.addressCopiedTimeout = setTimeout(() => {
             if(this.addressCopiedAlertEnabled){
                this.addressCopiedAlertEnabled = false
             }
          }, 250)
          

    }

 

}