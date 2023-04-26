use super::models::DatabaseSystembolagetProduct;
use crate::systembolaget::SystembolagetProduct;
use sqlx::sqlite::SqliteQueryResult;
use sqlx::{Connection, SqliteConnection};
use std::env;

pub struct Database {
    connection: SqliteConnection,
}

impl Database {
    pub async fn init() -> Self {
        Database {
            connection: SqliteConnection::connect(
                &env::var("DATABASE_URL").expect("'DATABASE_URL' is not configured"),
            )
            .await
            .expect("failed to create connection"),
        }
    }

    pub async fn create_tables(&mut self) -> Result<SqliteQueryResult, sqlx::Error> {
        let query = r#"
        CREATE TABLE IF NOT EXISTS products (
            product_id TEXT NOT NULL PRIMARY KEY, 
            product_number TEXT NOT NULL,
            product_name_bold TEXT NOT NULL, 
            product_name_thin TEXT, 
            producer_name TEXT, 
            supplier_name TEXT NOT NULL, 
            is_kosher INTEGER NOT NULL, 
            bottle_text TEXT NOT NULL, 
            is_organic INTEGER NOT NULL, 
            is_sustainable_choice INTEGER NOT NULL, 
            is_climate_smart_packaging INTEGER NOT NULL, 
            is_ethical INTEGER NOT NULL, ethical_label TEXT, 
            product_launch_date TEXT NOT NULL, 
            is_completely_out_of_stock INTEGER NOT NULL, 
            is_temporary_out_of_stock INTEGER NOT NULL, 
            alcohol_percentage TEXT NOT NULL,
            volume TEXT NOT NULL, 
            price TEXT NOT NULL, 
            country TEXT NOT NULL, 
            origin_level1 TEXT, 
            origin_level2 TEXT, 
            category_level1 TEXT, 
            category_level2 TEXT, 
            category_level3 TEXT, 
            category_level4 TEXT, 
            assortment_text TEXT NOT NULL, 
            is_manufacturing_country INTEGER NOT NULL, 
            is_regional_restricted INTEGER NOT NULL, 
            packaging_level1 TEXT, 
            image TEXT, 
            is_discontinued INTEGER NOT NULL, 
            is_supplier_temporary_not_available INTEGER NOT NULL, 
            sugar_content INTEGER NOT NULL, 
            sugar_content_gram_per100ml TEXT NOT NULL, 
            apk TEXT NOT NULL,
            url TEXT NOT NULL
        );
       "#;
        sqlx::query(query).execute(&mut self.connection).await
    }

    #[allow(dead_code)]
    pub async fn fetch_all_products(
        &mut self,
    ) -> Result<Vec<DatabaseSystembolagetProduct>, sqlx::Error> {
        sqlx::query_as::<_, DatabaseSystembolagetProduct>("SELECT * FROM products")
            .fetch_all(&mut self.connection)
            .await
    }

    pub async fn insert_product(
        &mut self,
        product: SystembolagetProduct,
    ) -> Result<(), sqlx::Error> {
        let _tmp = sqlx::query_as::<_, DatabaseSystembolagetProduct>(
            r#"
            INSERT OR REPLACE INTO products (
                product_id,
                product_number,
                product_name_bold,
                product_name_thin,
                producer_name,
                supplier_name,
                is_kosher,
                bottle_text,
                is_organic,
                is_sustainable_choice,
                is_climate_smart_packaging,
                is_ethical,
                product_launch_date,
                is_completely_out_of_stock,
                is_temporary_out_of_stock,
                alcohol_percentage,
                volume,
                price,
                country,
                origin_level1,
                origin_level2,
                category_level1,
                category_level2,
                category_level3,
                category_level4,
                assortment_text,
                is_manufacturing_country,
                is_regional_restricted,
                packaging_level1,
                image,
                is_discontinued,
                is_supplier_temporary_not_available,
                sugar_content,
                sugar_content_gram_per100ml,
                apk,
                url
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34, $35, $36)
            RETURNING *
                        "#,
        )
        .bind(&product.product_id)
        .bind(&product.product_number)
        .bind(&product.product_name_bold)
        .bind(&product.product_name_thin)
        .bind(&product.producer_name)
        .bind(&product.supplier_name)
        .bind(&product.is_kosher)
        .bind(&product.bottle_text)
        .bind(&product.is_organic)
        .bind(&product.is_sustainable_choice)
        .bind(&product.is_climate_smart_packaging)
        .bind(&product.is_ethical)
        .bind(&product.product_launch_date)
        .bind(&product.is_completely_out_of_stock)
        .bind(&product.is_temporary_out_of_stock)
        .bind(&product.alcohol_percentage)
        .bind(&product.volume)
        .bind(&product.price)
        .bind(&product.country)
        .bind(&product.origin_level1)
        .bind(&product.origin_level2)
        .bind(&product.category_level1)
        .bind(&product.category_level2)
        .bind(&product.category_level3)
        .bind(&product.category_level4)
        .bind(&product.assortment_text)
        .bind(&product.is_manufacturing_country)
        .bind(&product.is_regional_restricted)
        .bind(&product.packaging_level1)
        .bind(&product.get_image())
        .bind(&product.is_discontinued)
        .bind(&product.is_supplier_temporary_not_available)
        .bind(&product.sugar_content)
        .bind(&product.sugar_content_gram_per100ml)
        .bind(&product.calculate_apk())
        .bind(&product.get_url())
        .fetch_one(&mut self.connection)
        .await?;

        Ok(())
    }
}
