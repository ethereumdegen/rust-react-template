CREATE TABLE oauth_requests (
    id SERIAL PRIMARY KEY,

    
    state TEXT UNIQUE NOT NULL,
    code_verifier TEXT  NOT NULL,  

    domain_name TEXT  NOT NULL,
     
     
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);


 