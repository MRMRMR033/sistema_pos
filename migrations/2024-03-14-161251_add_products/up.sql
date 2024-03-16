-- Your SQL goes here
CREATE TABLE products(
    id_product SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    bar_code VARCHAR NOT NULL,
    cost_price DECIMAL(10, 2) NOT NULL,
    sales_price DECIMAL(10, 2) NOT NULL,
    promotion_price DECIMAL(10, 2),
    stock BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);