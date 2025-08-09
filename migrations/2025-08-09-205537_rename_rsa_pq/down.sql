ALTER TABLE rsa_encrypts
RENAME COLUMN n_p TO p;

ALTER TABLE rsa_encrypts
RENAME COLUMN n_q TO q;

ALTER TABLE rsa_decrypts
RENAME COLUMN n_p TO p;

ALTER TABLE rsa_decrypts
RENAME COLUMN n_q TO q;
