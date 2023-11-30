import { makeObservable, observable, action } from "mobx"
export class SideMenuStore {
 
    active=true

  
      /* 
    menu =  [
      "NAVIGATION",
      {
        icon: "Home",
        title: "Dashboard",
        pathname: "/dashboard",
       
        
      },

      {
        icon: "MessageSquare",
        title: "Documentation",
        pathname: "/docs",
       
        
      },
     
      {
        icon: "MessageSquare",
        pathname: "/dashboard/transactions",
        title: "Transactions",
      },
     
     {
        icon: "CreditCard",
        title: "E-commerce",
        subMenu: [
          {
            icon: "",
            pathname: "/products",
            title: "Products",
          },
          {
            icon: "",
            pathname: "/product-detail",
            title: "Product Detail",
          },
          {
            icon: "",
            pathname: "/orders",
            title: "Orders",
          },
          {
            icon: "",
            pathname: "/order-detail",
            title: "Order Detail",
          },
        ],
      },*/
      
      
    /*   {
        icon: "HardDrive",
        pathname: "/error-page",
        title: "Error Page",
      }, */
     /* "USER INTERFACE",
      {
        icon: "Inbox",
        title: "Components",
        subMenu: [
          {
            icon: "",
            pathname: "/table",
            title: "Table",
            subMenu: [
              {
                icon: "",
                pathname: "/regular-table",
                title: "Regular Table",
              },
              {
                icon: "",
                pathname: "/tabulator",
                title: "Tabulator",
              },
            ],
          },
          {
            icon: "",
            title: "Overlay",
            subMenu: [
              {
                icon: "",
                pathname: "/modal",
                title: "Modal",
              },
              {
                icon: "",
                pathname: "/slide-over",
                title: "Slide Over",
              },
              {
                icon: "",
                pathname: "/notification",
                title: "Notification",
              },
            ],
          },
          {
            icon: "",
            pathname: "/tab",
            title: "Tab",
          },
         
        ],
      },
      */

      /*
      {
        icon: "HardDrive",
        title: "Widgets",
        subMenu: [
          {
            icon: "",
            pathname: "/chart",
            title: "Chart",
          },
          {
            icon: "",
            pathname: "/slider",
            title: "Slider",
          },
          {
            icon: "",
            pathname: "/image-zoom",
            title: "Image Zoom",
          },
        ]
      }

      
    ];*/
    constructor() { 

      this.active = true 

      makeObservable(this, {
    //    menu: observable,
        active: observable,

        toggle: action ,
          
      })  
    
  }//constructor


  toggle(){
    console.log('toggle side menu', this)
    this.active = !this.active;
  }


}