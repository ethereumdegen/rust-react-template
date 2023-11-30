import { makeObservable, observable, action } from "mobx"
export class HeaderStore {
 
    mobileNavOpen=false

  
      
    menu =  [ 
    ];
    constructor() { 

      this.mobileNavOpen = false 

      makeObservable(this, {
       
        mobileNavOpen: observable,

        toggleMobileNav: action ,
        setMobileNavOpen: action ,
          
      })  
    
  }//constructor


  toggleMobileNav(){
 
    this.mobileNavOpen = !this.mobileNavOpen;
  }

  setMobileNavOpen(open){
    this.mobileNavOpen = !!open
  }
}