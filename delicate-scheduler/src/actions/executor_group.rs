use super::prelude::*;

pub(crate) fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(show_executor_groups)
        .service(create_executor_group)
        .service(update_executor_group)
        .service(delete_executor_group);
}

#[post("/api/executor_group/create")]
async fn create_executor_group(
    web::Json(executor_group): web::Json<model::NewExecutorGroup>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    use db::schema::executor_group;

    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<usize>>::into(
            web::block(move || {
                diesel::insert_into(executor_group::table)
                    .values(&executor_group)
                    .execute(&conn)
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<()>::error())
}

#[post("/api/executor_group/list")]
async fn show_executor_groups(
    web::Json(query_params): web::Json<model::QueryParamsExecutorGroup>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<
            UnifiedResponseMessages<PaginateData<model::ExecutorGroup>>,
        >::into(
            web::block::<_, _, diesel::result::Error>(move || {
                let query_builder = model::ExecutorGroupQueryBuilder::query_all_columns();

                let executor_groups = query_params
                    .clone()
                    .query_filter(query_builder)
                    .paginate(query_params.page)
                    .load::<model::ExecutorGroup>(&conn)?;

                let per_page = query_params.per_page;
                let count_builder = model::ExecutorGroupQueryBuilder::query_count();
                let count = query_params
                    .query_filter(count_builder)
                    .get_result::<i64>(&conn)?;

                Ok(PaginateData::<model::ExecutorGroup>::default()
                    .set_data_source(executor_groups)
                    .set_page_size(per_page)
                    .set_total(count))
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<PaginateData<model::ExecutorGroup>>::error())
}

#[post("/api/executor_group/update")]
async fn update_executor_group(
    web::Json(executor_group): web::Json<model::UpdateExecutorGroup>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<usize>>::into(
            web::block(move || {
                diesel::update(&executor_group)
                    .set(&executor_group)
                    .execute(&conn)
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<usize>::error())
}
#[post("/api/executor_group/delete")]
async fn delete_executor_group(
    web::Json(model::ExecutorGroupId { executor_group_id }): web::Json<model::ExecutorGroupId>,
    pool: ShareData<db::ConnectionPool>,
) -> HttpResponse {
    use db::schema::executor_group::dsl::*;

    if let Ok(conn) = pool.get() {
        return HttpResponse::Ok().json(Into::<UnifiedResponseMessages<usize>>::into(
            web::block(move || {
                diesel::delete(executor_group.find(executor_group_id)).execute(&conn)
            })
            .await,
        ));
    }

    HttpResponse::Ok().json(UnifiedResponseMessages::<usize>::error())
}
