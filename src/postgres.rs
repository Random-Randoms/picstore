use anyhow::anyhow;

pub trait PgClient {
    async fn create_tables(&self) -> anyhow::Result<()>;
    async fn user_in_scope(&self, user: &str, scope: &str) -> anyhow::Result<Option<i64>>;
    async fn _picture_in_scope(&self, picture: i64, scope: &str) -> anyhow::Result<bool>;
    async fn user_scope_picture_valid(
        &self,
        user: &str,
        scope: &str,
        picture: i64,
    ) -> anyhow::Result<bool>;
    async fn add_picture(&self, scope: i64) -> anyhow::Result<i64>;
    async fn add_user(&self, user: &str) -> anyhow::Result<()>;
    async fn add_scope(&self, scope: &str) -> anyhow::Result<()>;
    async fn add_user_scope(&self, user: &str, scope: &str) -> anyhow::Result<()>;
    async fn delete_picture(&self, picture: i64) -> anyhow::Result<()>;
}

impl PgClient for tokio_postgres::Client {
    async fn create_tables(&self) -> anyhow::Result<()> {
        self.batch_execute(include_str!("sql/create-tables.sql"))
            .await?;
        Ok(())
    }

    async fn add_picture(&self, scope: i64) -> anyhow::Result<i64> {
        self.query(include_str!("sql/add-picture.sql"), &[&scope])
            .await?
            .first()
            .and_then(|rw| rw.get(0))
            .ok_or(anyhow!("Unexpected pg answer"))
    }

    async fn add_user(&self, user: &str) -> anyhow::Result<()> {
        self.query(include_str!("sql/add-user.sql"), &[&user])
            .await?;
        Ok(())
    }

    async fn add_scope(&self, scope: &str) -> anyhow::Result<()> {
        self.query(include_str!("sql/add-scope.sql"), &[&scope])
            .await?;
        Ok(())
    }

    async fn add_user_scope(&self, user: &str, scope: &str) -> anyhow::Result<()> {
        self.query(include_str!("sql/add-user-scope.sql"), &[&user, &scope])
            .await?;
        Ok(())
    }

    async fn user_in_scope(&self, user: &str, scope: &str) -> anyhow::Result<Option<i64>> {
        Ok(self
            .query(include_str!("sql/user-in-scope.sql"), &[&user, &scope])
            .await?
            .first()
            .and_then(|rw| rw.get(0)))
    }

    async fn _picture_in_scope(&self, picture: i64, scope: &str) -> anyhow::Result<bool> {
        self.query(
            include_str!("sql/picture-in-scope.sql"),
            &[&picture, &scope],
        )
        .await?
        .first()
        .and_then(|rw| rw.get(0))
        .ok_or(anyhow!("Unexpected pg answer"))
    }

    async fn user_scope_picture_valid(
        &self,
        user: &str,
        scope: &str,
        picture: i64,
    ) -> anyhow::Result<bool> {
        self.query(
            include_str!("sql/user-scope-picture-valid.sql"),
            &[&user, &scope, &picture],
        )
        .await?
        .first()
        .and_then(|rw| rw.get(0))
        .ok_or(anyhow!("Unexpected pg answer"))
    }

    async fn delete_picture(&self, picture: i64) -> anyhow::Result<()> {
        self.query(include_str!("sql/delete-picture.sql"), &[&picture])
            .await?;
        Ok(())
    }
}
