UPDATE caesar_encrypts SET completed_at = NULL;
UPDATE caesar_decrypts SET completed_at = NULL;
UPDATE caesar_attacks SET completed_at = NULL;
UPDATE diffie_hellman_exchanges SET completed_at = NULL;

ALTER TABLE caesar_encrypts
ADD COLUMN cipher TEXT,
ADD CONSTRAINT caesar_encryption_completed_check CHECK (cipher IS NULL AND completed_at IS NULL OR cipher IS NOT NULL AND completed_at IS NOT NULL);

ALTER TABLE caesar_decrypts
ADD COLUMN message TEXT,
ADD CONSTRAINT caesar_decryption_completed_check CHECK (message IS NULL AND completed_at IS NULL OR message IS NOT NULL AND completed_at IS NOT NULL);

ALTER TABLE caesar_attacks
ADD COLUMN key TEXT,
ADD CONSTRAINT caesar_attack_completed_check CHECK (key IS NULL AND completed_at IS NULL OR key IS NOT NULL AND completed_at IS NOT NULL);

ALTER TABLE diffie_hellman_exchanges
ADD COLUMN pk_user BIGINT,
ADD COLUMN k BIGINT,
ADD CONSTRAINT diffie_hellman_exchange_completed_check CHECK (pk_user IS NULL AND k IS NULL AND completed_at IS NULL OR pk_user IS NOT NULL AND k IS NOT NULL AND completed_at IS NOT NULL);
