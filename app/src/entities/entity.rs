use crate::schema::employee::Employee;
use serde::{Deserialize, Serialize};

impl From<Employee> for Entity {
    fn from(employee: Employee) -> Self {
        let first_name = employee.first_name.expect("firstname is set");
        let last_name = employee.last_name.expect("lastname is set");
        let email = format!("{}.{}@dunkins.com", first_name, last_name);
        let dob = String::from("1997-03-18"); // Todo manually update format

        // Todo: get the address by id
        Self {
            entity_type: "individual".to_string(),
            individual: Individual {
                first_name,
                last_name,
                dob,
                email,
                phone: "+15121231111".to_string(),
            },
            address: Address {
                line1: "3300 N Interstate 35".to_string(),
                line2: "".to_string(),
                city: "Austin".to_string(),
                state: "TX".to_string(),
                zip: "78705".to_string(),
            },
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Entity {
    #[serde(rename = "type")]
    pub entity_type: String,
    pub individual: Individual,
    pub address: Address,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Individual {
    pub first_name: String,
    pub last_name: String,
    pub dob: String,
    pub email: String,
    pub phone: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    pub line1: String,
    pub line2: String,
    pub city: String,
    pub state: String,
    pub zip: String,
}
