-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
    'ddf8994f-d522-4659-8d02-c5d479057be6',
    'admin',
    '$argon2id$v=19$m=15000,t=2,p=1$KhVPMzmeVhqBWUl73ODQRw$jD1zX11QBUapOvxhGfg7V3yLpGx5FQcS4n7gTPoD5jA'
)
