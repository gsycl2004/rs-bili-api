pub mod info;

#[derive(Clone,Copy,Debug)]
pub struct Live{
    room_id:i64
}


impl Live {
    pub fn from_room_id(room_id:i64) -> Live {
        Live{
            room_id,
        }

    }
}