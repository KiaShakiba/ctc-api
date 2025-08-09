ALTER TABLE rsa_encrypts
RENAME COLUMN p TO n_p;

ALTER TABLE rsa_encrypts
RENAME COLUMN q TO n_q;

ALTER TABLE rsa_decrypts
RENAME COLUMN p TO n_p;

ALTER TABLE rsa_decrypts
RENAME COLUMN q TO n_q;
