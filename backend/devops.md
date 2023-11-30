
### Devops



had to add chmod 755 permissions to all folders in chain for nginx 



### nginx config




# For frontend
server {
    listen 80;
    listen [::]:80;

    server_name tempova.com;


 # Set the root directory for this server block
    root /home/andy/deploy/tempo-monorepo/tempo-frontend/dist;

    # Default location block
    location / {
        try_files $uri $uri/ /index.html =404;

    }
}


## backend 
server {
  listen 80;
  listen [::]:80;

  server_name api.tempova.com;

  location / {
      proxy_pass http://localhost:8000/;
      proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
  }

}