
  
import favicon from '@/assets/images/warp.gif'
import homeImage from '@/assets/images/degen_tx_banner.png'

const config = {
    title: 'Rust React Template',
    
    favicon: favicon,
    homeImage: homeImage,
     


   

    dashboardMenu: [
        "NAVIGATION",

          
            {
              icon: "Home",
              title: "Dashboard",
              pathname: "/dashboard",
             
           /*   subMenu: [
              {  icon: "",
                pathname: "/chart",
                title: "Chart"
              }
              ]*/


            },
 

            {
              icon: "MessageSquare",
              title: "Invoices",
              pathname: "/dashboard/invoices",             
              
            },
 
      
            
    ]
      


  
    

}



export default config;
//module.exports = config;
