CREATE TABLE caesar_encryptions (
	id SERIAL PRIMARY KEY,
	user_id INTEGER NOT NULL
		REFERENCES users(id)
		ON UPDATE CASCADE
		ON DELETE CASCADE,
	key INTEGER NOT NULL,
	message TEXT NOT NULL,
	cipher TEXT,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
	completed_at TIMESTAMP WITH TIME ZONE
	CONSTRAINT caesar_encryption_completed_check CHECK (cipher IS NULL AND completed_at IS NULL OR cipher IS NOT NULL AND completed_at IS NOT NULL)
);

CREATE TABLE caesar_decryptions (
	id SERIAL PRIMARY KEY,
	user_id INTEGER NOT NULL
		REFERENCES users(id)
		ON UPDATE CASCADE
		ON DELETE CASCADE,
	key INTEGER NOT NULL,
	message TEXT,
	cipher TEXT NOT NULL,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
	completed_at TIMESTAMP WITH TIME ZONE
	CONSTRAINT caesar_decryption_completed_check CHECK (message IS NULL AND completed_at IS NULL OR message IS NOT NULL AND completed_at IS NOT NULL)
);

CREATE TABLE caesar_attacks (
	id SERIAL PRIMARY KEY,
	user_id INTEGER NOT NULL
		REFERENCES users(id)
		ON UPDATE CASCADE
		ON DELETE CASCADE,
	key INTEGER,
	message TEXT NOT NULL,
	cipher TEXT NOT NULL,
	created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
	completed_at TIMESTAMP WITH TIME ZONE
	CONSTRAINT caesar_attack_completed_check CHECK (key IS NULL AND completed_at IS NULL OR key IS NOT NULL AND completed_at IS NOT NULL)
);
