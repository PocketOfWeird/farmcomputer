use rusted_cypher::cypher::result::Row;
use uuid::Uuid;

#[derive(juniper::GraphQLObject)]
pub struct Plant {
    /// The unique id of the plant
    id: Uuid,
    /// The name of the plant
    name: String,
    /// The genus, or family of the plant
    genus: String
}
impl Plant {
    pub fn mapper(row: &Row) -> Plant {
        let id: String = row.get("n.id").unwrap();
        return Plant {
            id: Uuid::parse_str(&id).unwrap(),
            name: row.get("n.name").unwrap(),
            genus: row.get("n.genus").unwrap(),
        }
    }
    pub fn get_one(id: Uuid) -> String {
        return format!("MATCH (n {{ id: {:?} }}) RETURN n.id, n.name, n.genus", id.to_string());
    }
    pub fn add_one(name: String, genus: String) -> String {
        let id = Uuid::new_v4();
        return format!("CREATE (n:Plant {{ id: {:?}, name: {:?}, genus: {:?}  }}) RETURN n.id, n.name, n.genus", id.to_string(), name.to_owned(), genus.to_owned());
    }
}
