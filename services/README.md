

Rust-based microservices which read and write SQL data.


 


## Services to build 
 


 https://github.com/twitterdev/Twitter-API-v2-sample-code/blob/main/Manage-Tweets/create_tweet.js


 https://twitter.com/i/oauth2/authorize?response_type=code&client_id=WG4xR0ZnTXU4ZS1JZTZjbGctaWk6MTpjaQ&redirect_uri=http://localhost:8080/api/callback&scope=tweet.read%20users.read%tweet.write%20offline.access&state=state&code_challenge=challenge&code_challenge_method=plain


 curl -X POST "https://api.twitter.com/oauth/request_token" \
  -H "User-Agent: tempova' HTTP Client" \
  -H "Host: api.twitter.com" \
  -H "Accept: */*" \
  -H "Authorization: OAuth oauth_callback=\"http%3A%2F%2Flocalhost%2Fsign-in-with-twitter%2F\", oauth_consumer_key=\"cChZNFj6T5R0TigYB9yd1w\", oauth_nonce=\"WG4xR0ZnTXU4ZS1JZTZjbGctaWk6MTpjaQ\", oauth_signature=\"F1Li3tvehgcraF8DMJ7OyxO4w9Y%3D\", oauth_signature_method=\"HMAC-SHA1\", oauth_timestamp=\"1318467427\", oauth_version=\"1.0\""




- allow for s3 image uploading later ? 