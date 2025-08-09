ALTER TABLE caesar_encrypts
DROP CONSTRAINT caesar_encryption_completed_check,
DROP COLUMN cipher;

ALTER TABLE caesar_decrypts
DROP CONSTRAINT caesar_decryption_completed_check,
DROP COLUMN message;

ALTER TABLE caesar_attacks
DROP CONSTRAINT caesar_attack_completed_check,
DROP COLUMN key;

ALTER TABLE diffie_hellman_exchanges
DROP CONSTRAINT diffie_hellman_exchange_completed_check,
DROP COLUMN pk_user,
DROP COLUMN k;
