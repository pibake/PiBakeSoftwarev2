use std::path::Path;

use uuid::Uuid;
use chrono::prelude::*;

pub struct Uuid {
    uuid: Uuid,
    deploy_date: DateTime<Local>,
    file: File
}

impl Uuid {
    pub fn new() -> Uuid {
        Uuid {
            
        }
    }

    pub fn does_file_exist() -> bool {
        let path = Path::new("uuid.txt");
        path.exists()
    }

    fn generate_deployment_date() {
        Local.now()
    }
    
    fn generate_uuid() -> Uuid {
        Uuid::new_v4()
    }
}

