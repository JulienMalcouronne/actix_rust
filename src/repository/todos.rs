use crate::entity::{prelude::*, todo};
use actix_web::web::Json;
use log::debug;
use sea_orm::{entity::*, query::*, DeleteResult, DeriveEntityModel};
use sea_orm::{ActiveValue::NotSet, DatabaseConnection, EntityTrait, Set};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TodoRequest {
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Clone)]
pub struct TodosRepository {
    pub db_conn: DatabaseConnection,
}

impl TodosRepository {
    pub async fn get_todos(&self) -> Vec<todo::Model> {
        Todo::find()
            .all(&self.db_conn)
            .await
            .expect("Failed to get todos")
    }

    pub async fn get_todo_by_id(&self, id: i32) -> Option<todo::Model> {
        Todo::find_by_id(id)
            .one(&self.db_conn)
            .await
            .expect("Failed to get todo")
    }

    pub async fn create_todo(&self, new_todo: Json<TodoRequest>) -> Option<todo::Model> {
        let todo = todo::ActiveModel {
            id: NotSet,
            title: ActiveValue::Set(new_todo.title.to_owned()),
            completed: ActiveValue::Set(new_todo.completed.to_owned()),
        };

        let todo: todo::Model = todo.insert(&self.db_conn).await.unwrap();
        debug!("Creatd todo: todo{}", todo.title);
        return todo.into();
    }

    pub async fn update_todo(&self, id: i32, new_todo: Json<TodoRequest>) -> Option<todo::Model> {
        let todo = Todo::find_by_id(id)
            .one(&self.db_conn)
            .await
            .expect("Failed to get todo");
        let mut todo: todo::ActiveModel = todo.unwrap().into();
        todo.title = ActiveValue::Set(new_todo.title.to_owned());
        todo.completed = ActiveValue::Set(new_todo.completed.to_owned());

        let todo: todo::Model = todo.update(&self.db_conn).await.unwrap();
        debug!("Updated todo: todo{}", todo.title);
        return todo.into();
    }

    pub async fn delete_todo_by_id(&self, id: i32) -> DeleteResult {
        let res: DeleteResult = Todo::delete_by_id(id).exec(&self.db_conn).await.unwrap();
        return res.into();
        // let todo = Todo::find_by_id(id)
        //     .one(&self.db)
        //     .await
        //     .expect("Failed to get todo");
        // let todo: todo::ActiveModel = todo.unwrap().into();
        // let result = todo.delete(&self.db).await.unwrap();
        // debug!("Deleted todo: todo{}", todo.title);
        // return result;
    }
}
