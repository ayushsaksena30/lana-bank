use es_entity::clock::ClockHandle;
use sqlx::PgPool;

use es_entity::*;

use super::{TermsTemplateId, entity::*, error::*};

#[derive(EsRepo, Clone)]
#[es_repo(
    entity = "TermsTemplate",
    err = "TermsTemplateError",
    columns(name(ty = "String", list_by)),
    tbl_prefix = "core"
)]
pub struct TermsTemplateRepo {
    pool: PgPool,
    clock: ClockHandle,
}

impl TermsTemplateRepo {
    pub fn new(pool: &PgPool, clock: ClockHandle) -> Self {
        Self {
            pool: pool.clone(),
            clock,
        }
    }

    pub async fn list_all(&self) -> Result<Vec<TermsTemplate>, TermsTemplateError> {
        let mut templates = Vec::new();
        let mut next = Some(es_entity::PaginatedQueryArgs::default());

        while let Some(query) = next.take() {
            let mut ret = self.list_by_name(query, es_entity::ListDirection::Ascending).await?;

            templates.append(&mut ret.entities);
            next = ret.into_next_query();
        }

        Ok(templates)
    }
}
