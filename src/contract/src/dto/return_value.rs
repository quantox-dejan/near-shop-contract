use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct ReturnValue<T>
where
    T: Default,
{
    pub result: T,
    pub success: bool,
}
