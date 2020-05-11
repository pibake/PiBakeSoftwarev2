use std::path::Path;
use std::time::SystemTime;

use uuid::Uuid;

pub struct TempUuid {
    pub uuid: Uuid,
    deploy_date: SystemTime,
    file: String
}

impl TempUuid {
    pub fn new() -> TempUuid {
        TempUuid {
            uuid: Uuid::new_v4(),
            deploy_date: SystemTime::now(),
            file: "uuid.txt".to_string(),
        }
    }

    fn does_file_exist(&self) -> bool {
        let path = Path::new("uuid.txt");
        path.exists()
    }

    pub fn read_uuid_file(&self) {

    }
}
