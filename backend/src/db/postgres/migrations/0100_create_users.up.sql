CREATE TABLE users (
    id SERIAL PRIMARY KEY,

    email TEXT , 
    google_oauth_uuid TEXT , 
    public_address eth_address , 

    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

 