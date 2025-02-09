use crate::entities::cities::{self, ActiveModel, Entity as Cities, Model};
use crate::services::weather_service::LatLong;
use sea_orm::prelude::DateTimeUtc;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, QueryOrder, QuerySelect,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),
}

pub struct CityRepository {
    db: DatabaseConnection,
}

impl CityRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn save_search(
        &self,
        name: String,
        coords: &LatLong,
        timestamp: Option<DateTimeUtc>,
    ) -> Result<Model, RepositoryError> {
        let now = timestamp.unwrap_or_else(chrono::Utc::now);

        // Create a new active model
        let city = ActiveModel {
            name: Set(name),
            lat: Set(coords.latitude),
            long: Set(coords.longitude),
            created_at: Set(now),
            ..Default::default()
        };

        // Insert and return the created model
        let result = city.insert(&self.db).await?;
        Ok(result)
    }

    pub async fn get_recent_searches(&self, limit: u64) -> Result<Vec<Model>, RepositoryError> {
        let cities = Cities::find()
            .order_by_desc(cities::Column::CreatedAt)
            .limit(limit)
            .all(&self.db)
            .await?;

        Ok(cities)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{Database, DatabaseConnection};
    use sea_orm_migration::MigratorTrait;

    async fn setup_test_db() -> DatabaseConnection {
        let db = Database::connect("sqlite::memory:")
            .await
            .expect("Failed to create test database");

        // Run migrations
        ::migration::Migrator::up(&db, None)
            .await
            .expect("Failed to run migrations");

        db
    }

    #[tokio::test]
    async fn test_save_and_retrieve_search() {
        let db = setup_test_db().await;
        let repo = CityRepository::new(db);
        let base_time = chrono::Utc::now();

        // Test data
        let first_city = "London".to_string();
        let first_coords = LatLong {
            latitude: 51.5074,
            longitude: -0.1278,
        };
        let timestamp = base_time + chrono::Duration::seconds(10);

        // Save search
        let saved = repo
            .save_search(first_city.clone(), &first_coords, Some(timestamp))
            .await
            .unwrap();
        assert_eq!(saved.name, first_city);
        assert!((saved.lat - first_coords.latitude).abs() < f32::EPSILON);
        assert!((saved.long - first_coords.longitude).abs() < f32::EPSILON);

        // Save another search
        let second_city = "Paris".to_string();
        let second_coords = LatLong {
            latitude: 48.8566,
            longitude: 2.3522,
        };
        let timestamp = base_time + chrono::Duration::seconds(20);

        repo.save_search(second_city.clone(), &second_coords, Some(timestamp))
            .await
            .unwrap();

        // Get recent searches
        let recent = repo.get_recent_searches(10).await.unwrap();
        assert_eq!(recent.len(), 2);

        // Check order (most recent first)
        assert_eq!(
            recent[0].name, second_city,
            "Most recent search should be Paris"
        );
        assert_eq!(
            recent[1].name, first_city,
            "Second most recent search should be London"
        );
    }

    #[tokio::test]
    async fn test_get_recent_searches_respects_limit() {
        let db = setup_test_db().await;
        let repo = CityRepository::new(db);
        let base_time = chrono::Utc::now();

        // Insert multiple cities
        for i in 0..5 {
            let city_name = format!("City{i}");
            let coords = LatLong {
                latitude: 0.0,
                longitude: 0.0,
            };

            let timestamp = base_time + chrono::Duration::seconds(From::from(i));

            repo.save_search(city_name, &coords, Some(timestamp))
                .await
                .unwrap();
        }

        // Test limit
        let recent = repo.get_recent_searches(3).await.unwrap();
        assert_eq!(recent.len(), 3, "Should only return 3 results");

        assert_eq!(recent[0].name, "City4", "Most recent should be City4");
        assert_eq!(
            recent[1].name, "City3",
            "Second most recent should be City3"
        );
        assert_eq!(recent[2].name, "City2", "Third most recent should be City2");
    }
}
