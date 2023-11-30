 #[derive(Debug)]
 pub struct AuthScopes(Vec<String>);

impl AuthScopes {
    // Converts an optional comma-separated string into a vector of scopes
    pub fn new_from_opt_string(opt: Option<String>) -> Self {
        let scopes = opt.unwrap_or_default();
        Self( scopes
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect() )
    }

    // Returns a vector of scopes that every authenticated user should have
    pub fn default_authed_user_scopes() -> Self {
       Self(  vec![
            "browse_products".into(),
            "read_orders".into(),
            "read_order_items".into(),
            "read_products".into(),
             "write_orders".into(),
        ] )
    }

    // Returns a vector of scopes specific to vendor roles
    pub fn vendor_scopes() -> Self {
       Self(   vec![
            "write_products".into(), //only vendors can add new products to a shop
        ] )
    }

    // Returns a vector of scopes based on the user's roles
    pub fn get_scopes_from_user_roles(roles: Vec<String>) -> Self{
        let is_vendor = roles.contains(&"vendor".into());
        
        let mut scopes = Self::default_authed_user_scopes().to_vec();
        
        if is_vendor {
            scopes.append(&mut Self::vendor_scopes().to_vec());
        }
        
       Self( scopes )
    }

    // You can add other methods that act on an instance of AuthScopes
    // For example, converting AuthScopes to String
    pub fn to_string(&self) -> String {
        self.0.join(", ")
    }
    
      pub fn to_vec(self) -> Vec<String> {
        self.0 
    }
}


/*
pub fn convert_scopes_string_to_array(opt: Option<String>) -> Vec<String> {
    let scopes = opt.unwrap_or_default();
    scopes
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}




pub fn default_authed_user_scopes() -> Vec<String> {
    
    vec![ 
        "browse_products".into(),
        "read_orders".into(),
        "read_order_items".into(),
        "read_products".into(),
        "write_orders".into(),   
    ]
    
}


pub fn vendor_scopes() -> Vec<String> {
    
    vec![
         
        "write_products".into(), //only vendors can add new products to a shop 
        
    ]
    
}

pub fn get_scopes_from_user_roles(roles: Vec<String>) -> Vec<String> {
    let is_vendor = roles.contains(&"vendor".into());
    
    let mut roles = default_authed_user_scopes();
    
    
    if is_vendor {
        roles.append( & mut vendor_scopes()   )
    }
    
    roles
} 
*/