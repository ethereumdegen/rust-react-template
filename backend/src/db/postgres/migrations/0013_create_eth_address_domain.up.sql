CREATE DOMAIN eth_address AS varchar(42)
CHECK (
    VALUE LIKE '0x%' AND
    LENGTH(VALUE) = 42  
   -- AND SUBSTRING(VALUE FOR 2) ~ '^[a-fA-F0-9]{40}$'
);