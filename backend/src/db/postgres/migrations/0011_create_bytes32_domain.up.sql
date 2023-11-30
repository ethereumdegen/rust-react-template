CREATE DOMAIN bytes32 AS varchar(66)
CHECK (
    VALUE LIKE '0x%' AND
    LENGTH(VALUE) = 66 
   -- AND SUBSTRING(VALUE FOR 2) ~ '^[a-fA-F0-9]{40}$'
);