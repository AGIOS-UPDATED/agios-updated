-- Create connections table
CREATE TABLE IF NOT EXISTS connections (
    id VARCHAR(255) PRIMARY KEY,
    provider VARCHAR(50) NOT NULL,
    access_token TEXT,
    refresh_token TEXT,
    expires_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create accounts table
CREATE TABLE IF NOT EXISTS accounts (
    id VARCHAR(255) PRIMARY KEY,
    connection_id VARCHAR(255) REFERENCES connections(id) ON DELETE CASCADE,
    account_number VARCHAR(255),
    account_type VARCHAR(50),
    currency VARCHAR(3),
    balance DECIMAL(20, 2),
    available DECIMAL(20, 2),
    name VARCHAR(255),
    official_name VARCHAR(255),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create transactions table
CREATE TABLE IF NOT EXISTS transactions (
    id VARCHAR(255) PRIMARY KEY,
    connection_id VARCHAR(255) REFERENCES connections(id) ON DELETE CASCADE,
    account_id VARCHAR(255) REFERENCES accounts(id) ON DELETE CASCADE,
    amount DECIMAL(20, 2),
    currency VARCHAR(3),
    description TEXT,
    merchant_name VARCHAR(255),
    merchant_category VARCHAR(255),
    transaction_type VARCHAR(50),
    transaction_date DATE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create institutions table
CREATE TABLE IF NOT EXISTS institutions (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    logo TEXT,
    country VARCHAR(2),
    primary_color VARCHAR(7),
    url TEXT,
    oauth_support BOOLEAN DEFAULT false,
    products JSONB,
    provider VARCHAR(50) NOT NULL,
    last_update TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create institution_usage table
CREATE TABLE IF NOT EXISTS institution_usage (
    id SERIAL PRIMARY KEY,
    institution_id VARCHAR(255) REFERENCES institutions(id) ON DELETE CASCADE,
    action VARCHAR(50) NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
