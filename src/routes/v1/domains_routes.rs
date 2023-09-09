use crate::db::db::ConfigMonkeyDb;
use crate::services::domains_service::{self, DomainsServiceError};
use chrono::{DateTime, Utc};
use rocket::http::Status;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{delete, get, post};
use rocket_db_pools::Connection;

use super::dtos::{PaginatedListDto, PaginationDto};
use super::errors::RoutesError;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GetDomainDto {
    pub slug: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateDomainDto {
    slug: String,
}

fn to_http_status(error: &DomainsServiceError) -> Status {
    match error {
        DomainsServiceError::DuplicateSlug => Status::Conflict,
        DomainsServiceError::InvalidSlug => Status::BadRequest,
        DomainsServiceError::NotFound => Status::NotFound,
        DomainsServiceError::NotEmpty => Status::UnprocessableEntity,
        _ => Status::InternalServerError,
    }
}

#[derive(Responder)]
#[response(status = 201, content_type = "json")]
pub struct CreateDomainSuccess(Json<GetDomainDto>);

#[post("/v1/domains", format = "application/json", data = "<input>")]
pub async fn create_domain(
    db: Connection<ConfigMonkeyDb>,
    input: Json<CreateDomainDto>,
) -> Result<CreateDomainSuccess, RoutesError> {
    let result = domains_service::create_domain(db, input.slug.as_str()).await;

    return match result {
        Ok(domain) => Ok(CreateDomainSuccess(Json(GetDomainDto {
            slug: domain.slug,
            created_at: domain.created_at,
        }))),
        Err(err) => Err(RoutesError(to_http_status(&err), err.code(), err.message())),
    };
}

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct GetDomainsResponse(Json<PaginatedListDto<GetDomainDto>>);

#[get("/v1/domains?<limit>&<offset>")]
pub async fn get_domains(
    db: Connection<ConfigMonkeyDb>,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<GetDomainsResponse, RoutesError> {
    let result = domains_service::get_domains(db, limit, offset).await;

    return match result {
        Ok(domains) => {
            let mut result = vec![];

            for domain in domains.items {
                result.push(GetDomainDto {
                    slug: domain.slug,
                    created_at: domain.created_at,
                });
            }
            Ok(GetDomainsResponse(Json(PaginatedListDto {
                data: result,
                pagination: PaginationDto {
                    count: domains.count,
                    offset: domains.offset,
                    limit: domains.limit,
                    next: if let Some(next_offset) = domains.next_offset {
                        Some(format!(
                            "/v1/domains?limit={}&offset={}",
                            domains.limit, next_offset
                        ))
                    } else {
                        None
                    },
                    prev: if let Some(prev_offset) = domains.prev_offset {
                        Some(format!(
                            "/v1/domains?limit={}&offset={}",
                            domains.limit, prev_offset
                        ))
                    } else {
                        None
                    },
                },
            })))
        }
        Err(err) => Err(RoutesError(to_http_status(&err), err.code(), err.message())),
    };
}

#[derive(Responder)]
#[response(status = 204, content_type = "json")]
pub struct DeleteDomainSuccess(());

#[delete("/v1/domains/<slug>")]
pub async fn delete_domain(
    db: Connection<ConfigMonkeyDb>,
    slug: &str,
) -> Result<DeleteDomainSuccess, RoutesError> {
    let result = domains_service::delete_domain(db, slug).await;

    return match result {
        Ok(()) => Ok(DeleteDomainSuccess(())),
        Err(err) => Err(RoutesError(to_http_status(&err), err.code(), err.message())),
    };
}
