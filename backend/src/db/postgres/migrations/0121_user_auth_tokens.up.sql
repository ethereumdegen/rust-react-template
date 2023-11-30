CREATE TABLE user_auth_tokens (
    id SERIAL PRIMARY KEY,

    parent_user_id INTEGER references users(id), 
 
    auth_token TEXT, 
    service_name TEXT,
  
    expires_at TIMESTAMPTZ NOT NULL , 
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

 