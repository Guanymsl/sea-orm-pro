//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize)]
#[sea_orm(table_name = "customer")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub customer_id: i32,
    #[serde(deserialize_with = "super::utils::bool_from_int")]
    pub name_style: bool,
    pub title: Option<String>,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub suffix: Option<String>,
    pub company_name: Option<String>,
    pub sales_person: Option<String>,
    pub email_address: Option<String>,
    pub phone: Option<String>,
    pub password_hash: String,
    pub password_salt: String,
    #[sea_orm(unique)]
    pub rowguid: Uuid,
    #[serde(deserialize_with = "super::utils::date_time_from_str")]
    pub created_date: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::customer_address::Entity")]
    CustomerAddress,
    #[sea_orm(has_many = "super::sales_order_header::Entity")]
    SalesOrderHeader,
}

impl Related<super::customer_address::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CustomerAddress.def()
    }
}

impl Related<super::sales_order_header::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::SalesOrderHeader.def()
    }
}

impl Related<super::address::Entity> for Entity {
    fn to() -> RelationDef {
        super::customer_address::Relation::Address.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::customer_address::Relation::Customer.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {
    #[sea_orm(entity = "super::customer_address::Entity")]
    CustomerAddress,
    #[sea_orm(entity = "super::sales_order_header::Entity")]
    SalesOrderHeader,
    #[sea_orm(entity = "super::address::Entity")]
    Address,
}
