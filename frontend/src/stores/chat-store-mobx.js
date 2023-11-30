


 
 
import { useState } from "react";
 

 
 
 
import { makeObservable, observable, action } from "mobx"
 


export class ChatStore {
    
    
    connectedAppName=undefined
    connectedAppAuthToken=undefined
    
    connected = false
  
    constructor() {
 
      
        makeObservable(this, {
            
            connectedAppName: observable,
            connectedAppAuthToken: observable,
            
            connected: observable ,
             
 
            connect: action,
            disconnect: action,

            saveState: action,
            loadState: action 
        })
        
    }
 
   
    async connect( {appName, appAuthToken }  ) {

        //mock for now 

        this.connectedAppName = appName
        this.connectedAppAuthToken = appAuthToken
        
        this.connected = true 
        this.saveState();
    }
 
    async disconnect(    ) {
 

        this.connectedAppName = undefined
        this.connectedAppAuthToken = undefined
        
        this.connected = false 
        this.saveState();
    }





  saveState() {
    const state = {
      // Include the properties you want to save in localStorage
      connectedAppName: this.connectedAppName,
      connectedAppAuthToken: this.connectedAppAuthToken,
      connected: this.connected 
    };

    localStorage.setItem("chatStore", JSON.stringify(state));
  }

  // Load state from localStorage
  loadState() {
    const storedState = localStorage.getItem("chatStore");
    if (storedState) {
      const state = JSON.parse(storedState);
      // Update the store properties with the loaded state

      //if the loaded state is too old DONT load it, delete it?
      if (this.authTokenExpiresAt && new Date(this.authTokenExpiresAt) > Date.now()) {
        console.log("tried to load expired state");
      } else {
        this.connectedAppName = state.connectedAppName;
        this.connectedAppAuthToken = state.connectedAppAuthToken;
        this.connected = state.connected; 
      }
    }
  }





}



 


 